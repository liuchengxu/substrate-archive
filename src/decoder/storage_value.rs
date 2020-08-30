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

/// An object to track the currently used extrinsic weight in a block.
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Debug)]
pub struct ExtrinsicsWeight {
    normal: Weight,
    operational: Weight,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum UncleEntryItem<BlockNumber, Hash, Author> {
    InclusionHeight(BlockNumber),
    Uncle(Hash, Option<Author>),
}

type Hash = sp_core::H256;
type BlockNumber = u32;
type AccountId = chainx_runtime::AccountId;

pub fn try_decode_storage_value(any_ty: &str, encoded: Vec<u8>) -> Result<(), codec::Error> {
    /*
    use frame_system::AccountInfo;
    use pallet_balances::AccountData;
    use polkadot_primitives::v1::{AccountIndex, Balance};
    */

    // TODO: use a macro?
    match any_ty {
        // "AccountInfo<T::Index, T::AccountData>" => {
        // let decoded: AccountInfo<AccountIndex, AccountData<Balance>> = generic_decode(encoded)?;
        // println!("decoded value:{:?}", decoded);
        // }
        "T::BlockNumber" => {
            let decoded: BlockNumber = super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }
        "Vec<T::BlockNumber>" => {
            let decoded: Vec<BlockNumber> = super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }
        "Multiplier" => {
            let decoded: sp_runtime::FixedU128 = super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }
        "bool" => {
            let decoded: bool = super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }
        "u32" | "EventIndex" => {
            let decoded: u32 = super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }
        "u64" | "T::Moment" => {
            let decoded: u64 = super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }
        "T::Hash" => {
            let decoded: Hash = super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }
        "Vec<T::Hash>" => {
            let decoded: Vec<sp_core::H256> = super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }
        "weights::ExtrinsicsWeight" => {
            let decoded: ExtrinsicsWeight = super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }
        "Vec<EventRecord<T::Event, T::Hash>>" => {
            let decoded: Vec<EventRecord<chainx_runtime::Event, Hash>> =
                super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }
        "Vec<UncleEntryItem<T::BlockNumber, T::Hash, T::AccountId>>" => {
            let decoded: Vec<UncleEntryItem<BlockNumber, Hash, AccountId>> =
                super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }
        "EraRewardPoints<T::AccountId>" => {
            let decoded: pallet_staking::EraRewardPoints<AccountId> =
                super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }
        "ActiveEraInfo" => {
            let decoded: pallet_staking::ActiveEraInfo = super::generic_decode(encoded)?;
            println!("decoded: {:?}", decoded);
        }

        _ => {
            println!("Unknown value type: {:?}", any_ty);
            return Err("Unknown value type".into());
        }
    }

    Ok(())
}
