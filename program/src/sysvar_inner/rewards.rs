//! This sysvar is deprecated and unused.
use super::SysvarSerialize;
use serde_derive::{Deserialize, Serialize};
pub use solana_sdk_ids::sysvar::rewards::{check_id, id, ID};
use {super::Sysvar, solana_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Rewards);

#[repr(C)]
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Rewards {
    pub validator_point_value: f64,
    pub unused: f64,
}
impl Rewards {
    pub fn new(validator_point_value: f64) -> Self {
        Self {
            validator_point_value,
            unused: 0.0,
        }
    }
}
impl Sysvar for Rewards {}
impl SysvarSerialize for Rewards {}
