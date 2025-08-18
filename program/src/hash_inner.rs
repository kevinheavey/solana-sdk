use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use bytemuck_derive::{Pod, Zeroable};
use serde_derive::{Deserialize, Serialize};
use std::string::ToString;
use {
    super::sanitize_inner::Sanitize,
    core::{
        fmt,
        str::{from_utf8_unchecked, FromStr},
    },
};

/// Size of a hash in bytes.
pub const HASH_BYTES: usize = 32;
/// Maximum string length of a base58 encoded hash.
pub const MAX_BASE58_LEN: usize = 44;

/// A hash; the 32-byte output of a hashing algorithm.
///
/// This struct is used most often in `solana-sdk` and related crates to contain
/// a [SHA-256] hash, but may instead contain a [blake3] hash.
///
/// [SHA-256]: https://en.wikipedia.org/wiki/SHA-2
/// [blake3]: https://github.com/BLAKE3-team/BLAKE3
#[derive(
    Clone,
    Copy,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    BorshSerialize,
    BorshDeserialize,
    BorshSchema,
    Pod,
    Zeroable,
    Serialize,
    Deserialize,
)]
#[repr(transparent)]
pub struct Hash(pub(crate) [u8; HASH_BYTES]);

impl Sanitize for Hash {}

impl From<[u8; HASH_BYTES]> for Hash {
    fn from(from: [u8; 32]) -> Self {
        Self(from)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

fn write_as_base58(f: &mut fmt::Formatter, h: &Hash) -> fmt::Result {
    let mut out = [0u8; MAX_BASE58_LEN];
    let len = five8::encode_32(&h.0, &mut out) as usize;
    // any sequence of base58 chars is valid utf8
    let as_str = unsafe { from_utf8_unchecked(&out[..len]) };
    f.write_str(as_str)
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_as_base58(f, self)
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_as_base58(f, self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseHashError {
    WrongSize,
    Invalid,
}

impl core::error::Error for ParseHashError {}

impl fmt::Display for ParseHashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseHashError::WrongSize => f.write_str("string decoded to wrong size for hash"),
            ParseHashError::Invalid => f.write_str("failed to decoded string to hash"),
        }
    }
}

impl FromStr for Hash {
    type Err = ParseHashError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use five8::DecodeError;
        if s.len() > MAX_BASE58_LEN {
            return Err(ParseHashError::WrongSize);
        }
        let mut bytes = [0; HASH_BYTES];
        five8::decode_32(s, &mut bytes).map_err(|e| match e {
            DecodeError::InvalidChar(_) => ParseHashError::Invalid,
            DecodeError::TooLong
            | DecodeError::TooShort
            | DecodeError::LargestTermTooHigh
            | DecodeError::OutputTooLong => ParseHashError::WrongSize,
        })?;
        Ok(Self::from(bytes))
    }
}

impl Hash {
    pub const fn new_from_array(hash_array: [u8; HASH_BYTES]) -> Self {
        Self(hash_array)
    }

    /// unique Hash for tests and benchmarks.
    pub fn new_unique() -> Self {
        use super::atomic_u64_inner::AtomicU64;
        static I: AtomicU64 = AtomicU64::new(1);

        let mut b = [0u8; HASH_BYTES];
        let i = I.fetch_add(1);
        b[0..8].copy_from_slice(&i.to_le_bytes());
        Self::new_from_array(b)
    }

    pub const fn to_bytes(self) -> [u8; HASH_BYTES] {
        self.0
    }

    pub const fn as_bytes(&self) -> &[u8; HASH_BYTES] {
        &self.0
    }
}
