//! Storage Key/Value decoding.

pub mod metadata;
pub mod storage_value;
pub mod types;

use std::collections::HashMap;

use codec::Decode;
use frame_metadata::{DecodeDifferent, StorageEntryType, StorageHasher};

use self::metadata::{Metadata, StorageMetadata};

/// module prefix and storage prefix both use twx_128 hasher. One twox_128
/// hasher is 32 chars in hex string, i.e, the prefix length is 32 * 2.
pub const PREFIX_LENGTH: usize = 32 * 2;

/// Map of StorageKey prefix (module_prefix++storage_prefix) in hex string to StorageMetadata.
///
/// So that we can know about the StorageMetadata given a complete StorageKey.
#[derive(Debug, Clone)]
pub struct StorageMetadataLookupTable(pub HashMap<String, StorageMetadata>);

impl From<Metadata> for StorageMetadataLookupTable {
    fn from(metadata: Metadata) -> Self {
        Self(
            metadata
                .modules
                .into_iter()
                .map(|(_, module_metadata)| {
                    module_metadata
                        .storage
                        .into_iter()
                        .map(|(_, storage_metadata)| {
                            let storage_prefix = storage_metadata.prefix();
                            (hex::encode(storage_prefix.0), storage_metadata)
                        })
                })
                .flatten()
                .collect(),
        )
    }
}

/// Transparent type of decoded StorageKey.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransparentStorageType {
    Plain {
        /// "u32"
        value_ty: String,
    },
    Map {
        /// value of key, e.g, "be5ddb1579b72e84524fc29e78609e3caf42e85aa118ebfe0b0ad404b5bdd25f" for T::AccountId
        key: String,
        /// type of value, e.g., "AccountInfo<T::Index, T::AccountData>"
        value_ty: String,
    },
    DoubleMap {
        key1: String,
        key1_ty: String,
        key2: String,
        key2_ty: String,
        value_ty: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransparentStorageKey {
    pub module_prefix: String,
    pub storage_prefix: String,
    pub ty: TransparentStorageType,
}

impl TransparentStorageKey {
    pub fn get_value_type(&self) -> String {
        match &self.ty {
            TransparentStorageType::Plain { value_ty } => value_ty.into(),
            TransparentStorageType::Map { value_ty, .. } => value_ty.into(),
            TransparentStorageType::DoubleMap { value_ty, .. } => value_ty.into(),
        }
    }
}

fn build_transparent_storage_key(
    storage_metadata: &StorageMetadata,
    ty: TransparentStorageType,
) -> TransparentStorageKey {
    TransparentStorageKey {
        module_prefix: String::from(&storage_metadata.module_prefix),
        storage_prefix: String::from(&storage_metadata.storage_prefix),
        ty,
    }
}

impl StorageMetadataLookupTable {
    /// Returns the StorageMetadata given the `prefix` of a StorageKey.
    pub fn lookup(&self, prefix: &str) -> Option<&StorageMetadata> {
        self.0.get(prefix)
    }

    /// Converts `storage_key` in hex string to a _readable_ format.
    pub fn parse_storage_key(&self, storage_key: String) -> Option<TransparentStorageKey> {
        let key_length = storage_key.chars().count();
        if key_length < 64 {
            log::error!("Unknown StorageKey: {:?}", storage_key);
            return None;
        }

        let storage_prefix = &storage_key[..PREFIX_LENGTH];

        if let Some(storage_metadata) = self.lookup(storage_prefix) {
            match &storage_metadata.ty {
                StorageEntryType::Plain(value) => Some(TransparentStorageKey {
                    module_prefix: String::from(&storage_metadata.module_prefix),
                    storage_prefix: String::from(&storage_metadata.storage_prefix),
                    ty: TransparentStorageType::Plain {
                        value_ty: as_decoded_type(value.clone()),
                    },
                }),
                StorageEntryType::Map { hasher, value, .. } => match hasher {
                    StorageHasher::Twox64Concat | StorageHasher::Blake2_128Concat => {
                        let hashed_key_concat = &storage_key[PREFIX_LENGTH..];
                        let hash_length = hash_length_of(hasher);
                        let _hashed_key = &hashed_key_concat[..hash_length];
                        let key = &hashed_key_concat[hash_length..];

                        let transparent_ty = TransparentStorageType::Map {
                            key: key.into(),
                            value_ty: as_decoded_type(value.clone()),
                        };

                        Some(build_transparent_storage_key(
                            &storage_metadata,
                            transparent_ty,
                        ))
                    }
                    _ => unreachable!("All Map storage should use foo_concat hasher"),
                },
                StorageEntryType::DoubleMap {
                    hasher,
                    key1,
                    key2,
                    value,
                    key2_hasher,
                } => {
                    // hashed_key1 ++ key1 ++ hashed_key2 ++ key2
                    let hashed_key_concat = &storage_key[PREFIX_LENGTH..];
                    match hasher {
                        StorageHasher::Twox64Concat | StorageHasher::Blake2_128Concat => {
                            let key1_hash_length = hash_length_of(hasher);

                            // key1 ++ hashed_key2 ++ key2
                            let key1_hashed_key2_key2 = &hashed_key_concat[key1_hash_length..];

                            let key1_ty = as_decoded_type(key1.clone());

                            if let Some(key1_length) = get_key1_length(key1_ty.clone()) {
                                let key1 = &key1_hashed_key2_key2[..key1_length];
                                let hashed_key2_key2 = &key1_hashed_key2_key2[key1_length..];

                                match key2_hasher {
                                    StorageHasher::Twox64Concat
                                    | StorageHasher::Blake2_128Concat => {
                                        let key2_hash_length = hash_length_of(key2_hasher);
                                        let raw_key2 = &hashed_key2_key2[key2_hash_length..];

                                        let key2_ty = as_decoded_type(key2.clone());

                                        let transparent_ty = TransparentStorageType::DoubleMap {
                                            key1: key1.into(),
                                            key1_ty,
                                            key2: raw_key2.into(),
                                            key2_ty,
                                            value_ty: as_decoded_type(value.clone()),
                                        };

                                        Some(build_transparent_storage_key(
                                            &storage_metadata,
                                            transparent_ty,
                                        ))
                                    }
                                    _ => unreachable!(
                                    "All DoubleMap storage should use foo_concat hasher for key2"
                                ),
                                }
                            } else {
                                log::error!("Can not infer the length of DoubleMap's key1");
                                None
                            }
                        }
                        _ => unreachable!(
                            "All DoubleMap storage should use foo_concat hasher for key1"
                        ),
                    }
                }
            }
        } else {
            log::error!(
                "Can not find the StorageMetadata from lookup table for storage_key: {:?},
                prefix: {:?}",
                storage_key,
                storage_prefix
            );
            None
        }
    }
}

/// TODO: ensure all key1 in DoubleMap are included in this table.
///
/// NOTE: The lucky thing is that key1 of double_map normally uses the fixed size encoding.
fn get_double_map_key1_length_table() -> HashMap<String, u32> {
    let mut double_map_key1_length_table = HashMap::new();
    // For the test metadata:
    // [
    //  ("Kind", "OpaqueTimeSlot"),
    //  ("T::AccountId", "[u8; 32]"),
    //  ("EraIndex", "T::AccountId"),
    //  ("EraIndex", "T::AccountId"),
    //  ("EraIndex", "T::AccountId"),
    //  ("EraIndex", "T::AccountId"),
    //  ("EraIndex", "T::AccountId"),
    //  ("SessionIndex", "AuthIndex"),
    //  ("SessionIndex", "T::ValidatorId")
    // ]
    double_map_key1_length_table.insert(String::from("T::AccountId"), 64);
    // u32 hex::encode(1u32.encode()).chars().count()
    double_map_key1_length_table.insert(String::from("SessionIndex"), 8);
    // u32
    double_map_key1_length_table.insert(String::from("EraIndex"), 8);
    // Kind = [u8; 16]
    double_map_key1_length_table.insert(String::from("Kind"), 32);
    double_map_key1_length_table.insert(String::from("Chain"), 2);
    double_map_key1_length_table
}

/// Returns the length of key1 for a DoubleMap.
///
/// For key1 ++ hashed_key2 ++ key2, we already know the length of hashed_key2, plus
/// the length of key1, we could also infer the length of key2.
fn get_key1_length(key1_ty: String) -> Option<usize> {
    let table = get_double_map_key1_length_table();
    table.get(&key1_ty).copied().map(|x| x as usize)
}

/// Returns the length of this hasher in hex.
fn hash_length_of(hasher: &StorageHasher) -> usize {
    match hasher {
        StorageHasher::Blake2_128 => 32,
        StorageHasher::Blake2_256 => 32 * 2,
        StorageHasher::Blake2_128Concat => 32,
        StorageHasher::Twox128 => 32,
        StorageHasher::Twox256 => 32 * 2,
        StorageHasher::Twox64Concat => 16,
        StorageHasher::Identity => unreachable!(),
    }
}

pub fn generic_decode<T: codec::Decode>(encoded: Vec<u8>) -> Result<T, codec::Error> {
    Decode::decode(&mut encoded.as_slice())
}

/// Filter out (key1, key2) pairs of all DoubleMap.
pub fn filter_double_map(metadata: &Metadata) -> Vec<(String, String)> {
    metadata
        .modules
        .iter()
        .map(|(_, module_metadata)| {
            module_metadata
                .storage
                .iter()
                .filter_map(|(_, storage_metadata)| {
                    if let StorageEntryType::DoubleMap {
                        ref key1, ref key2, ..
                    } = storage_metadata.ty
                    {
                        let key1_ty = as_decoded_type(key1.clone());
                        let key2_ty = as_decoded_type(key2.clone());
                        Some((key1_ty, key2_ty))
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .collect()
}

pub fn filter_double_map_key1_types(metadata: &Metadata) -> Vec<String> {
    let keys_map: HashMap<String, String> = filter_double_map(metadata).into_iter().collect();
    let key1_type_set = keys_map.keys();
    key1_type_set.into_iter().cloned().collect()
}

/// Converts the inner of `DecodeDifferent::Decoded(_)` to String.
fn as_decoded_type<B: 'static, O: 'static + Into<String>>(value: DecodeDifferent<B, O>) -> String {
    match value {
        DecodeDifferent::Encode(_b) => unreachable!("TODO: really unreachable?"),
        DecodeDifferent::Decoded(o) => o.into(),
    }
}

fn get_value_type(ty: &StorageEntryType) -> String {
    match ty {
        StorageEntryType::Plain(ref value) => as_decoded_type(value.clone()),
        StorageEntryType::Map { ref value, .. } => as_decoded_type(value.clone()),
        StorageEntryType::DoubleMap { ref value, .. } => as_decoded_type(value.clone()),
    }
}

/// Filters all the types of storage value.
pub fn filter_storage_value_types(metadata: &Metadata) -> Vec<String> {
    let mut value_types = metadata
        .modules
        .iter()
        .map(|(_, module_metadata)| {
            module_metadata
                .storage
                .iter()
                .map(|(_, storage_metadata)| get_value_type(&storage_metadata.ty))
        })
        .flatten()
        .collect::<Vec<_>>();

    value_types.sort();
    value_types.dedup();
    value_types
}
