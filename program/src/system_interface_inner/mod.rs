//! The System program interface.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod error;
pub mod instruction;

/// Maximum permitted size of account data (10 MiB).
///
// SBF program entrypoint assumes that the max account data length
// will fit inside a u32. If this constant no longer fits in a u32,
// the entrypoint deserialization code in the SDK must be updated.
pub const MAX_PERMITTED_DATA_LENGTH: u64 = 10 * 1024 * 1024;

/// Maximum permitted size of new allocations per transaction, in bytes.
///
/// The value was chosen such that at least one max sized account could be created,
/// plus some additional resize allocations.
pub const MAX_PERMITTED_ACCOUNTS_DATA_ALLOCATIONS_PER_TRANSACTION: i64 =
    MAX_PERMITTED_DATA_LENGTH as i64 * 2;

pub mod program {
    super::super::declare_id!("11111111111111111111111111111111");
}
