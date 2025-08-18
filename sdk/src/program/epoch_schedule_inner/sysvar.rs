pub use super::super::sdk_ids::sysvar::epoch_schedule::{check_id, id, ID};
use {super::EpochSchedule, crate::impl_sysvar_id};

impl_sysvar_id!(EpochSchedule);
