pub use solana_sdk_ids::sysvar::epoch_rewards::{check_id, id, ID};
use {super::EpochRewards, super::super::impl_sysvar_id};

impl_sysvar_id!(EpochRewards);
