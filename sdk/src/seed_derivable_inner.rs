//! The interface by which keys are derived.
use {crate::derivation_path_inner::DerivationPath, std::error};

/// The `SeedDerivable` trait defines the interface by which cryptographic keys/keypairs are
/// derived from byte seeds, derivation paths, and passphrases.
pub trait SeedDerivable: Sized {
    fn from_seed(seed: &[u8]) -> Result<Self, Box<dyn error::Error>>;
    fn from_seed_and_derivation_path(
        seed: &[u8],
        derivation_path: Option<DerivationPath>,
    ) -> Result<Self, Box<dyn error::Error>>;
    fn from_seed_phrase_and_passphrase(
        seed_phrase: &str,
        passphrase: &str,
    ) -> Result<Self, Box<dyn error::Error>>;
}
