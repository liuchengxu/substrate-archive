/*
pub fn try_decode_storage_value(any_ty: &str, encoded_hex_str: &str) -> Result<(), codec::Error> {
    /*
    use frame_system::AccountInfo;
    use pallet_balances::AccountData;
    use polkadot_primitives::v1::{AccountIndex, Balance};
    */

    let encoded = hex::decode(encoded_hex_str).unwrap();

    // TODO: use a macro?
    match any_ty {
        // "AccountInfo<T::Index, T::AccountData>" => {
        // let decoded: AccountInfo<AccountIndex, AccountData<Balance>> = generic_decode(encoded)?;
        // println!("decoded value:{:?}", decoded);
        // }
        _ => {
            println!("Unknown value type: {:?}", any_ty);
            return Err("Unknown value type".into());
        }
    }

    Ok(())
}
*/

use codec::{Decode, Encode};
use frame_support::weights::Weight;
use frame_system::{Event, EventRecord};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// An object to track the currently used extrinsic weight in a block.
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct ExtrinsicsWeight {
    normal: Weight,
    operational: Weight,
}

#[derive(Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
pub enum UncleEntryItem<BlockNumber, Hash, Author> {
    InclusionHeight(BlockNumber),
    Uncle(Hash, Option<Author>),
}

/// Information regarding the active era (era in used in session).
#[derive(Encode, Decode, Debug, Serialize, Deserialize)]
pub struct ActiveEraInfo {
    /// Index of era.
    pub index: EraIndex,
    /// Moment of start expressed as millisecond from `$UNIX_EPOCH`.
    ///
    /// Start can be none if start hasn't been set for the era yet,
    /// Start is set on the first on_finalize of the era to guarantee usage of `Time`.
    start: Option<u64>,
}

// frame_staking
#[derive(PartialEq, Encode, Decode, Default, Debug, Serialize, Deserialize)]
pub struct EraRewardPoints<AccountId: Ord> {
    /// Total number of points. Equals the sum of reward points for each validator.
    total: RewardPoint,
    /// The reward points earned by a given validator.
    individual: BTreeMap<AccountId, RewardPoint>,
}

// pallet_balances
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug, Serialize, Deserialize)]
pub struct AccountData<Balance> {
    pub free: Balance,
    pub reserved: Balance,
    pub misc_frozen: Balance,
    pub fee_frozen: Balance,
}

/// Type used to encode the number of references an account has.
pub type RefCount = u8;

// frame_system
#[derive(Clone, Eq, PartialEq, Default, Debug, Encode, Decode, Serialize, Deserialize)]
pub struct AccountInfo<Index, AccountData> {
    pub nonce: Index,
    pub refcount: RefCount,
    pub data: AccountData,
}

type EraIndex = u32;
type RewardPoint = u32;

type Hash = sp_core::H256;
type BlockNumber = u32;
type AccountIndex = u32;
type Balance = u128;
type AccountId = chainx_runtime::AccountId;

pub fn try_decode_storage_value(
    any_ty: &str,
    encoded: Vec<u8>,
) -> Result<serde_json::value::Value, codec::Error> {
    // TODO: use a macro?
    let value = match any_ty {
        "AccountInfo<T::Index, T::AccountData>" => {
            let decoded: AccountInfo<AccountIndex, AccountData<Balance>> =
                super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }
        "T::BlockNumber" => {
            let decoded: BlockNumber = super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }
        "Vec<T::BlockNumber>" => {
            let decoded: Vec<BlockNumber> = super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }
        "Multiplier" => {
            let decoded: sp_runtime::FixedU128 = super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }
        "bool" => {
            let decoded: bool = super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }
        "u32" | "EventIndex" => {
            let decoded: u32 = super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }
        "u64" | "T::Moment" => {
            let decoded: u64 = super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }
        "T::Hash" => {
            let decoded: Hash = super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }
        "Vec<T::Hash>" => {
            let decoded: Vec<sp_core::H256> = super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }
        "weights::ExtrinsicsWeight" => {
            let decoded: ExtrinsicsWeight = super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }
        "Vec<EventRecord<T::Event, T::Hash>>" => {
            let decoded: Vec<EventRecord<chainx_runtime::Event, Hash>> =
                super::generic_decode(encoded)?;
            println!("TODO: serialized decoded: {:?} ", decoded);
            return Err("Can not serialize this type".into());
            // serde_json::json!({ "value": decoded })
        }
        "Vec<UncleEntryItem<T::BlockNumber, T::Hash, T::AccountId>>" => {
            let decoded: Vec<UncleEntryItem<BlockNumber, Hash, AccountId>> =
                super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }
        "EraRewardPoints<T::AccountId>" => {
            let decoded: EraRewardPoints<AccountId> = super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }
        "ActiveEraInfo" => {
            let decoded: ActiveEraInfo = super::generic_decode(encoded)?;
            serde_json::json!({ "value": decoded })
        }

        _ => {
            println!("Unknown value type: {:?}", any_ty);
            return Err("Unknown value type".into());
        }
    };

    Ok(value)
}
