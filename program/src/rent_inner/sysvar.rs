pub use super::super::sdk_ids::sysvar::rent::{check_id, id, ID};
use {super::Rent, super::super::impl_sysvar_id};

impl_sysvar_id!(Rent);
