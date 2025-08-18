pub use solana_sdk_ids::sysvar::rent::{check_id, id, ID};
use {super::Rent, solana_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Rent);
