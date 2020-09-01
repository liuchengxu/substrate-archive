use super::types::*;

use frame_system::EventRecord;

// use node_template_runtime::Event;

macro_rules! try_decode_and_as_json {
    ( $value_ty:expr, $encoded:expr => $($decoded_ty_string: expr => $decoded_ty:ty;)+ ) => {

        match $value_ty {
            $(
                $decoded_ty_string => {
                    let decoded: $decoded_ty = super::generic_decode($encoded)?;
                    serde_json::json!({ "value": decoded })

                }
            )+
            _ => {
                println!("Unknown value type: {:?}", $value_ty);
                return Err("Unknown value type".into());
            }
        }
    };
}

pub fn try_decode_storage_value(
    any_ty: &str,
    encoded: Vec<u8>,
) -> Result<serde_json::value::Value, codec::Error> {
    let value = try_decode_and_as_json! {
    any_ty, encoded =>
        "u32" => u32;
        "u64" => u64;
        "bool" => bool;

        "T::Hash" => Hash;
        "T::Moment" => u64;
        "T::BlockNumber" => BlockNumber;
        "T::AccountId" => AccountId;

        "EventIndex"  => u32;
        "Multiplier" => sp_runtime::FixedU128;

        "Vec<T::Hash>" => Vec<Hash>;
        "Vec<T::BlockNumber>" => Vec<BlockNumber>;
        "weights::ExtrinsicsWeight" => ExtrinsicsWeight;
        "Vec<UncleEntryItem<T::BlockNumber, T::Hash, T::AccountId>>" => Vec<UncleEntryItem<BlockNumber, Hash, AccountId>>;
        "EraRewardPoints<T::AccountId>" => EraRewardPoints<AccountId>;
        "ActiveEraInfo" => ActiveEraInfo;

        "(BalanceOf<T>, Vec<T::AccountId>)" => (Balance, Vec<AccountId>);
        // "AccountInfo<T::Index, T::AccountData>" => AccountInfo<AccountIndex, AccountData<Balance>>;
        // "Vec<EventRecord<T::Event, T::Hash>>" => Vec<EventRecord<Event, Hash>>;
    };

    Ok(value)
}
