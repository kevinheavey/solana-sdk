pub use solana_sdk_ids::sysvar::slot_hashes::{check_id, id, ID};
use {super::SlotHashes, super::super::impl_sysvar_id};

impl_sysvar_id!(SlotHashes);
