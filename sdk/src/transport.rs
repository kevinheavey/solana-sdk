//! Defines the [`TransportError`] type.

#![cfg(feature = "full")]
#[deprecated(since = "2.1.0", note = "Use solana_transaction_error crate instead")]
pub use crate::program::transaction_error_inner::{TransportError, TransportResult as Result};
