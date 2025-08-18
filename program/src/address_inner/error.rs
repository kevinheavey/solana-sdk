use {core::fmt, super::super::program_error_inner::ProgramError};

#[derive(Debug, Clone, PartialEq, Eq, serde_derive::Serialize)]
pub enum AddressError {
    /// Length of the seed is too long for address generation
    MaxSeedLengthExceeded,
    InvalidSeeds,
    IllegalOwner,
}

impl core::error::Error for AddressError {}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddressError::MaxSeedLengthExceeded => {
                f.write_str("Length of the seed is too long for address generation")
            }
            AddressError::InvalidSeeds => {
                f.write_str("Provided seeds do not result in a valid address")
            }
            AddressError::IllegalOwner => f.write_str("Provided owner is not allowed"),
        }
    }
}

impl From<u64> for AddressError {
    fn from(error: u64) -> Self {
        match error {
            0 => AddressError::MaxSeedLengthExceeded,
            1 => AddressError::InvalidSeeds,
            2 => AddressError::IllegalOwner,
            _ => panic!("Unsupported AddressError"),
        }
    }
}

impl From<AddressError> for ProgramError {
    fn from(error: AddressError) -> Self {
        match error {
            AddressError::MaxSeedLengthExceeded => Self::MaxSeedLengthExceeded,
            AddressError::InvalidSeeds => Self::InvalidSeeds,
            AddressError::IllegalOwner => Self::IllegalOwner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde_derive::Serialize)]
pub enum ParseAddressError {
    WrongSize,
    Invalid,
}

impl core::error::Error for ParseAddressError {}

impl fmt::Display for ParseAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseAddressError::WrongSize => f.write_str("String is the wrong size"),
            ParseAddressError::Invalid => f.write_str("Invalid Base58 string"),
        }
    }
}

impl From<core::convert::Infallible> for ParseAddressError {
    fn from(_: core::convert::Infallible) -> Self {
        unreachable!("Infallible uninhabited");
    }
}
