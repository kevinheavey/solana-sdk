pub use super::super::sdk_ids::sysvar::slot_hashes::{check_id, id, ID};
use {crate::impl_sysvar_id, super::SlotHashes};

impl_sysvar_id!(SlotHashes);
