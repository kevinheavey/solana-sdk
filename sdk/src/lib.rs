//! The Solana host and client SDK.
//!
//! This is the base library for all off-chain programs that interact with
//! Solana or otherwise operate on Solana data structures. On-chain programs
//! instead use the [`solana-program`] crate, the modules of which are
//! re-exported by this crate, like the relationship between the Rust
//! `core` and `std` crates. As much of the functionality of this crate is
//! provided by `solana-program`, see that crate's documentation for an
//! overview.
//!
//! [`solana-program`]: https://docs.rs/solana-program
//!
//! Many of the modules in this crate are primarily of use to the Solana runtime
//! itself. Additional crates provide capabilities built on `solana-sdk`, and
//! many programs will need to link to those crates as well, particularly for
//! clients communicating with Solana nodes over RPC.
//!
//! Such crates include:
//!
//! - [`solana-client`] - For interacting with a Solana node via the [JSON-RPC API][json].
//! - [`solana-cli-config`] - Loading and saving the Solana CLI configuration file.
//! - [`solana-clap-utils`] - Routines for setting up the CLI using [`clap`], as
//!   used by the Solana CLI. Includes functions for loading all types of
//!   signers supported by the CLI.
//!
//! [`solana-client`]: https://docs.rs/solana-client
//! [`solana-cli-config`]: https://docs.rs/solana-cli-config
//! [`solana-clap-utils`]: https://docs.rs/solana-clap-utils
//! [json]: https://solana.com/docs/rpc
//! [`clap`]: https://docs.rs/clap

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

// Allows macro expansion of `use ::solana_sdk::*` to work within this crate
extern crate self as solana_sdk;

#[cfg(feature = "borsh")]
pub use crate::program::borsh1;
#[deprecated(since = "2.2.0", note = "Use `solana-message` crate instead")]
pub use crate::program::message_inner as message;
#[cfg(not(target_os = "solana"))]
pub use crate::program::program_stubs;
pub use crate::program::{
    account_info, big_mod_exp, blake3, bpf_loader, bpf_loader_deprecated, clock, config,
    debug_account_data, ed25519_program, epoch_rewards, epoch_schedule, fee_calculator,
    incinerator, instruction, keccak, lamports, native_token, program_error,
    program_option, program_pack, rent, secp256k1_program, serialize_utils, slot_hashes,
    slot_history, stable_layout, syscalls, sysvar,
};
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `solana-signer` crate instead")]
pub use crate::signer_inner::signers;
pub mod entrypoint;
pub mod entrypoint_deprecated;
pub mod epoch_info_inner;
pub mod example_mocks;
pub mod hash;
pub mod log;
pub mod native_loader;
pub mod pubkey;
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `solana-shred-version` crate instead")]
pub use shred_version_inner as shred_version;
pub mod account_inner;
mod derivation_path_inner;
pub mod epoch_rewards_hasher_inner;
pub mod fee_structure_inner;
mod hard_forks_inner;
pub mod inflation_inner;
mod keypair_inner;
pub mod offchain_message_inner;
pub mod packet_inner;
pub mod presigner_inner;
pub mod program;
mod seed_derivable_inner;
mod seed_phrase_inner;
pub mod serde_inner;
pub mod shred_version_inner;
pub mod signature;
pub mod signature_inner;
pub mod signer;
pub mod signer_inner;
pub mod time_utils_inner;
pub mod transaction;
mod transaction_inner;
pub mod transport;

#[deprecated(since = "2.1.0", note = "Use `solana-program-memory` crate instead")]
pub use crate::program::program_memory;
#[deprecated(since = "2.1.0", note = "Use `solana-sanitize` crate instead")]
pub use crate::program::sanitize_inner as sanitize;
#[deprecated(since = "2.1.0", note = "Use `solana-serde-varint` crate instead")]
pub use crate::program::serde_varint_inner as serde_varint;
#[deprecated(since = "2.1.0", note = "Use `solana-short-vec` crate instead")]
pub use crate::program::short_vec;
#[deprecated(since = "2.1.0", note = "Use `solana-account` crate instead")]
pub use account_inner as account;
#[deprecated(
    since = "2.1.0",
    note = "Use `solana_account::state_traits` crate instead"
)]
pub use account_inner::state_traits as account_utils;
#[deprecated(since = "2.2.0", note = "Use `solana-epoch-info` crate instead")]
pub use epoch_info_inner as epoch_info;
#[deprecated(
    since = "2.2.0",
    note = "Use `solana-epoch-rewards-hasher` crate instead"
)]
pub use epoch_rewards_hasher_inner as epoch_rewards_hasher;
#[deprecated(since = "2.2.0", note = "Use `solana-fee-structure` crate instead")]
pub use fee_structure_inner as fee;
#[deprecated(since = "2.1.0", note = "Use `solana-inflation` crate instead")]
pub use inflation_inner as inflation;
#[deprecated(
    since = "2.2.0",
    note = "Use `solana_message::inner_instruction` instead"
)]
pub use message::inner_instruction;
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `solana-offchain-message` crate instead")]
pub use offchain_message_inner as offchain_message;
#[deprecated(since = "2.2.0", note = "Use `solana-serde` crate instead")]
pub use serde_inner as deserialize_utils;
/// Convenience macro to declare a static public key and functions to interact with it.
///
/// Input: a single literal base58 string representation of a program's id
///
/// # Example
///
/// ```
/// # // wrapper is used so that the macro invocation occurs in the item position
/// # // rather than in the statement position which isn't allowed.
/// use std::str::FromStr;
/// use solana_sdk::{declare_id, pubkey::Pubkey};
///
/// # mod item_wrapper {
/// #   use solana_sdk::declare_id;
/// declare_id!("My11111111111111111111111111111111111111111");
/// # }
/// # use item_wrapper::id;
///
/// let my_id = Pubkey::from_str("My11111111111111111111111111111111111111111").unwrap();
/// assert_eq!(id(), my_id);
/// ```
pub use solana_sdk_macro::declare_id;
/// Convenience macro to define multiple static public keys.
pub use solana_sdk_macro::pubkeys;
#[deprecated(since = "2.2.0", note = "Use `solana-time-utils` crate instead")]
pub use time_utils_inner as timing;
#[cfg(feature = "full")]
#[deprecated(
    since = "2.2.0",
    note = "Use `solana_transaction::simple_vote_transaction_checker` instead"
)]
pub use transaction_inner::simple_vote_transaction_checker;

pub extern crate bs58;
