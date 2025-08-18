//! Address representation for Solana.
//!
//! An address is a sequence of 32 bytes, often shown as a base58 encoded string
//! (e.g. 14grJpemFaf88c8tiVb77W7TYg2W3ir6pfkKz3YjhhZ5).

#![allow(clippy::arithmetic_side_effects)]

pub mod error;
mod hasher;
pub mod syscalls;

use error::AddressError;
use error::ParseAddressError;
#[cfg(not(target_os = "solana"))]
pub use hasher::{AddressHasher, AddressHasherBuilder};

use bytemuck_derive::{Pod, Zeroable};
use core::str::FromStr;
use core::{
    array,
    convert::TryFrom,
    hash::{Hash, Hasher},
};
use serde_derive::{Deserialize, Serialize};
use std::vec::Vec;
use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    std::string::ToString,
};

/// Number of bytes in an address.
pub const ADDRESS_BYTES: usize = 32;
/// maximum length of derived `Address` seed
pub const MAX_SEED_LEN: usize = 32;
/// Maximum number of seeds
pub const MAX_SEEDS: usize = 16;
/// Maximum string length of a base58 encoded address.
const MAX_BASE58_LEN: usize = 44;

const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

/// The address of a [Solana account][acc].
///
/// Some account addresses are [ed25519] public keys, with corresponding secret
/// keys that are managed off-chain. Often, though, account addresses do not
/// have corresponding secret keys &mdash; as with [_program derived
/// addresses_][pdas] &mdash; or the secret key is not relevant to the operation
/// of a program, and may have even been disposed of. As running Solana programs
/// can not safely create or manage secret keys, the full [`Keypair`] is not
/// defined in `solana-program` but in `solana-sdk`.
///
/// [acc]: https://solana.com/docs/core/accounts
/// [ed25519]: https://ed25519.cr.yp.to/
/// [pdas]: https://solana.com/docs/core/cpi#program-derived-addresses
/// [`Keypair`]: https://docs.rs/solana-sdk/latest/solana_sdk/signer/keypair/struct.Keypair.html
#[repr(transparent)]
#[derive(
    Clone,
    Copy,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    BorshSerialize,
    BorshDeserialize,
    BorshSchema,
    Pod,
    Zeroable,
    Deserialize,
    Serialize,
)]
pub struct Address(pub(crate) [u8; 32]);

impl super::sanitize_inner::Sanitize for Address {}

impl FromStr for Address {
    type Err = ParseAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use five8::DecodeError;
        if s.len() > MAX_BASE58_LEN {
            return Err(ParseAddressError::WrongSize);
        }
        let mut bytes = [0; ADDRESS_BYTES];
        five8::decode_32(s, &mut bytes).map_err(|e| match e {
            DecodeError::InvalidChar(_) => ParseAddressError::Invalid,
            DecodeError::TooLong
            | DecodeError::TooShort
            | DecodeError::LargestTermTooHigh
            | DecodeError::OutputTooLong => ParseAddressError::WrongSize,
        })?;
        Ok(Address(bytes))
    }
}

/// Custom impl of Hash for Address.
///
/// This allows us to skip hashing the length of the address
/// which is always the same anyway.
impl Hash for Address {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.as_array());
    }
}

impl From<&Address> for Address {
    #[inline]
    fn from(value: &Address) -> Self {
        *value
    }
}

impl From<[u8; 32]> for Address {
    #[inline]
    fn from(from: [u8; 32]) -> Self {
        Self(from)
    }
}

impl TryFrom<&[u8]> for Address {
    type Error = array::TryFromSliceError;

    #[inline]
    fn try_from(address: &[u8]) -> Result<Self, Self::Error> {
        <[u8; 32]>::try_from(address).map(Self::from)
    }
}

impl TryFrom<Vec<u8>> for Address {
    type Error = Vec<u8>;

    #[inline]
    fn try_from(address: Vec<u8>) -> Result<Self, Self::Error> {
        <[u8; 32]>::try_from(address).map(Self::from)
    }
}
impl TryFrom<&str> for Address {
    type Error = ParseAddressError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Address::from_str(s)
    }
}

// If target_os = "solana", then this panics so there are no dependencies.
// When target_os != "solana", this should be opt-in so users
// don't need the curve25519 dependency.
#[allow(clippy::used_underscore_binding)]
pub fn bytes_are_curve_point<T: AsRef<[u8]>>(_bytes: T) -> bool {
    #[cfg(not(target_os = "solana"))]
    {
        let Ok(compressed_edwards_y) =
            curve25519_dalek::edwards::CompressedEdwardsY::from_slice(_bytes.as_ref())
        else {
            return false;
        };
        compressed_edwards_y.decompress().is_some()
    }
    #[cfg(target_os = "solana")]
    unimplemented!();
}

impl Address {
    pub const fn new_from_array(address_array: [u8; 32]) -> Self {
        Self(address_array)
    }

    /// Decode a string into an `Address`, usable in a const context
    pub const fn from_str_const(s: &str) -> Self {
        let id_array = five8_const::decode_32_const(s);
        Address::new_from_array(id_array)
    }

    /// Create an unique `Address` for tests and benchmarks.
    pub fn new_unique() -> Self {
        use super::atomic_u64_inner::AtomicU64;
        static I: AtomicU64 = AtomicU64::new(1);
        type T = u32;
        const COUNTER_BYTES: usize = core::mem::size_of::<T>();
        let mut b = [0u8; ADDRESS_BYTES];
        let mut i = I.fetch_add(1) as T;
        // use big endian representation to ensure that recent unique addresses
        // are always greater than less recent unique addresses.
        b[0..COUNTER_BYTES].copy_from_slice(&i.to_be_bytes());
        // fill the rest of the address with pseudorandom numbers to make
        // data statistically similar to real addresses.
        {
            let mut hash = std::hash::DefaultHasher::new();
            for slice in b[COUNTER_BYTES..].chunks_mut(COUNTER_BYTES) {
                hash.write_u32(i);
                i += 1;
                slice.copy_from_slice(&hash.finish().to_ne_bytes()[0..COUNTER_BYTES]);
            }
        }
        Self::from(b)
    }

    // If target_os = "solana", then the solana_sha256_hasher crate will use
    // syscalls which bring no dependencies.
    // When target_os != "solana", this should be opt-in so users
    // don't need the sha2 dependency.
    pub fn create_with_seed(
        base: &Address,
        seed: &str,
        owner: &Address,
    ) -> Result<Address, AddressError> {
        if seed.len() > MAX_SEED_LEN {
            return Err(AddressError::MaxSeedLengthExceeded);
        }

        let owner = owner.as_ref();
        if owner.len() >= PDA_MARKER.len() {
            let slice = &owner[owner.len() - PDA_MARKER.len()..];
            if slice == PDA_MARKER {
                return Err(AddressError::IllegalOwner);
            }
        }
        let hash = super::sha256_hasher_inner::hashv(&[base.as_ref(), seed.as_ref(), owner]);
        Ok(Address::from(hash.to_bytes()))
    }

    pub const fn to_bytes(self) -> [u8; 32] {
        self.0
    }

    /// Return a reference to the `Address`'s byte array.
    #[inline(always)]
    pub const fn as_array(&self) -> &[u8; 32] {
        &self.0
    }

    // If target_os = "solana", then this panics so there are no dependencies.
    // When target_os != "solana", this should be opt-in so users
    // don't need the curve25519 dependency.
    pub fn is_on_curve(&self) -> bool {
        bytes_are_curve_point(self)
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for Address {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

fn write_as_base58(f: &mut core::fmt::Formatter, p: &Address) -> core::fmt::Result {
    let mut out = [0u8; MAX_BASE58_LEN];
    let len = five8::encode_32(&p.0, &mut out) as usize;
    // any sequence of base58 chars is valid utf8
    let as_str = unsafe { core::str::from_utf8_unchecked(&out[..len]) };
    f.write_str(as_str)
}

impl core::fmt::Debug for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write_as_base58(f, self)
    }
}

impl core::fmt::Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write_as_base58(f, self)
    }
}

/// Convenience macro to define a static `Address` value.
///
/// Input: a single literal base58 string representation of an `Address`.
///
/// # Example
///
/// ```
/// use std::str::FromStr;
/// use solana_address::{address, Address};
///
/// static ID: Address = address!("My11111111111111111111111111111111111111111");
///
/// let my_id = Address::from_str("My11111111111111111111111111111111111111111").unwrap();
/// assert_eq!(ID, my_id);
/// ```
#[macro_export]
macro_rules! address {
    ($input:literal) => {
        $crate::address_inner::Address::from_str_const($input)
    };
}
