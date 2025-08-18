pub use solana_sdk_ids::sysvar::clock::{check_id, id, ID};
use {super::Clock, super::super::impl_sysvar_id};

impl_sysvar_id!(Clock);
