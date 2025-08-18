//! Functionality for public and private keys.
#![cfg(feature = "full")]

// legacy module paths
#[deprecated(
    since = "2.2.0",
    note = "Use solana_keypair::signable::Signable instead."
)]
pub use crate::keypair_inner::signable::Signable;
pub use {
    crate::signature_inner::{ParseSignatureError, Signature, SIGNATURE_BYTES},
    crate::signer::{keypair::*, null_signer::*, presigner::*, *},
};
