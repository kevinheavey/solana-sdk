//! Solana account addresses.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(clippy::arithmetic_side_effects)]

// If target_os = "solana", then this panics so there are no dependencies.
// When target_os != "solana", this should be opt-in so users
// don't need the curve25519 dependency.
pub use super::address_inner::bytes_are_curve_point;
pub use super::address_inner::{
    error::{AddressError as PubkeyError, ParseAddressError as ParsePubkeyError},
    Address as Pubkey, ADDRESS_BYTES as PUBKEY_BYTES, MAX_SEEDS, MAX_SEED_LEN,
};
#[cfg(not(target_os = "solana"))]
pub use super::address_inner::{
    AddressHasher as PubkeyHasher, AddressHasherBuilder as PubkeyHasherBuilder,
};
pub use crate::address as pubkey;
#[cfg(target_os = "solana")]
pub use super::address_inner::syscalls;

/// New random `Pubkey` for tests and benchmarks.
#[cfg(not(target_os = "solana"))]
pub fn new_rand() -> Pubkey {
    Pubkey::from(rand::random::<[u8; PUBKEY_BYTES]>())
}


/// Same as [`declare_id`] except that it reports that this ID has been deprecated.
#[macro_export]
macro_rules! declare_deprecated_id {
    ($pubkey:expr) => {
        /// The const program ID.
        pub const ID: $crate::pubkey::Pubkey = $crate::pubkey::Pubkey::from_str_const($pubkey);

        /// Returns `true` if given pubkey is the program ID.
        // TODO make this const once `derive_const` makes it out of nightly
        // and we can `derive_const(PartialEq)` on `Pubkey`.
        #[deprecated()]
        pub fn check_id(id: &$crate::pubkey::Pubkey) -> bool {
            id == &ID
        }

        /// Returns the program ID.
        #[deprecated()]
        pub const fn id() -> $crate::pubkey::Pubkey {
            ID
        }

        #[cfg(test)]
        #[test]
        #[allow(deprecated)]
        fn test_id() {
            assert!(check_id(&id()));
        }
    };
}
