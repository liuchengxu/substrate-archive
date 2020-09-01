use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub use node_template_runtime::AccountId;

pub type AccountIndex = u32;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Hash = sp_core::H256;

pub type EraIndex = u32;
pub type RewardPoint = u32;

// frame_support::weights::Weight;
pub type Weight = u64;

/// Type used to encode the number of references an account has.
pub type RefCount = u8;

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

// frame_system
#[derive(Clone, Eq, PartialEq, Default, Debug, Encode, Decode, Serialize, Deserialize)]
pub struct AccountInfo<Index, AccountData> {
    pub nonce: Index,
    pub refcount: RefCount,
    pub data: AccountData,
}
