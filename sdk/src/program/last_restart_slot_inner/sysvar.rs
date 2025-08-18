pub use super::super::sdk_ids::sysvar::last_restart_slot::{check_id, id, ID};
use {super::LastRestartSlot, crate::impl_sysvar_id};

impl_sysvar_id!(LastRestartSlot);
