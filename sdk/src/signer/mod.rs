#![cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `solana-presigner` crate instead")]
pub use crate::presigner_inner as presigner;
#[deprecated(since = "2.2.0", note = "Use `solana-seed-derivable` crate instead")]
pub use crate::seed_derivable_inner::SeedDerivable;
#[deprecated(since = "2.2.0", note = "Use `solana-signer` crate instead")]
pub use solana_signer::{
    null_signer, signers, unique_signers, EncodableKey, EncodableKeypair, Signer, SignerError,
};
pub mod keypair;
