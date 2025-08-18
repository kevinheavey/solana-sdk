pub use super::super::sdk_ids::sysvar::slot_history::{check_id, id, ID};
use {super::SlotHistory, crate::impl_sysvar_id};

impl_sysvar_id!(SlotHistory);
