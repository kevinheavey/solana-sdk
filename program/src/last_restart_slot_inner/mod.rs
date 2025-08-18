//! Information about the last restart slot (hard fork).
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod sysvar;

use solana_sdk_macro::CloneZeroed;

#[repr(C)]
#[derive(
    Debug, CloneZeroed, PartialEq, Eq, Default, serde_derive::Deserialize, serde_derive::Serialize,
)]
pub struct LastRestartSlot {
    /// The last restart `Slot`.
    pub last_restart_slot: u64,
}
