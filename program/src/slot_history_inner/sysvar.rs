pub use solana_sdk_ids::sysvar::slot_history::{check_id, id, ID};
use {super::SlotHistory, super::super::impl_sysvar_id};

impl_sysvar_id!(SlotHistory);
