//! Hashing with the [SHA-256] hash function, and a general [`Hash`] type.
//!
//! [SHA-256]: https://en.wikipedia.org/wiki/SHA-2
//! [`Hash`]: struct@Hash

#[cfg(not(target_os = "solana"))]
pub use super::sha256_hasher_inner::Hasher;
pub use {
    super::hash_inner::{Hash, ParseHashError, HASH_BYTES},
    super::sha256_hasher_inner::{hash, hashv},
};
