#[cfg(feature = "full")]
pub use solana_program::pubkey::new_rand;
#[cfg(target_os = "solana")]
pub use solana_program::pubkey::syscalls;
pub use solana_program::pubkey::{
    bytes_are_curve_point, ParsePubkeyError, Pubkey, PubkeyError, MAX_SEEDS, MAX_SEED_LEN,
    PUBKEY_BYTES,
};
