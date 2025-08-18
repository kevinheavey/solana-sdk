pub use solana_sdk_ids::sysvar::last_restart_slot::{check_id, id, ID};
use {super::LastRestartSlot, super::super::impl_sysvar_id};

impl_sysvar_id!(LastRestartSlot);
