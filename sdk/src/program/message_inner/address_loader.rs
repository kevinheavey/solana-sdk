use {
    super::super::super::transaction_error_inner::AddressLoaderError,
    super::v0::{LoadedAddresses, MessageAddressTableLookup},
};

pub trait AddressLoader: Clone {
    fn load_addresses(
        self,
        lookups: &[MessageAddressTableLookup],
    ) -> Result<LoadedAddresses, AddressLoaderError>;
}

#[derive(Clone)]
pub enum SimpleAddressLoader {
    Disabled,
    Enabled(LoadedAddresses),
}

impl AddressLoader for SimpleAddressLoader {
    fn load_addresses(
        self,
        _lookups: &[MessageAddressTableLookup],
    ) -> Result<LoadedAddresses, AddressLoaderError> {
        match self {
            Self::Disabled => Err(AddressLoaderError::Disabled),
            Self::Enabled(loaded_addresses) => Ok(loaded_addresses),
        }
    }
}
