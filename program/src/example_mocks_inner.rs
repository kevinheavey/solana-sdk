//! Mock types for use in examples.
//!
//! These represent APIs from crates that themselves depend on this crate, and
//! which are useful for illustrating the examples for APIs in this crate.
//!
//! Directly depending on these crates though would cause problematic circular
//! dependencies, so instead they are mocked out here in a way that allows
//! examples to appear to use crates that this crate must not depend on.
//!
//! Each mod here has the name of a crate, so that examples can be structured to
//! appear to import from that crate.

#![doc(hidden)]
#![allow(clippy::new_without_default)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod solana_rpc_client {
    pub mod rpc_client {
        use {
            super::super::{
                solana_rpc_client_api::client_error::Result as ClientResult,
                solana_sdk::{
                    account::Account, hash::Hash, pubkey::Pubkey, signature::Signature,
                    transaction::Transaction,
                },
            },
            std::{cell::RefCell, collections::HashMap, rc::Rc},
        };

        #[derive(Default)]
        pub struct RpcClient {
            get_account_responses: Rc<RefCell<HashMap<Pubkey, Account>>>,
        }

        impl RpcClient {
            pub fn new(_url: String) -> Self {
                RpcClient::default()
            }

            pub fn get_latest_blockhash(&self) -> ClientResult<Hash> {
                Ok(Hash::default())
            }

            pub fn send_and_confirm_transaction(
                &self,
                _transaction: &Transaction,
            ) -> ClientResult<Signature> {
                Ok(Signature)
            }

            pub fn get_minimum_balance_for_rent_exemption(
                &self,
                _data_len: usize,
            ) -> ClientResult<u64> {
                Ok(0)
            }

            pub fn get_account(&self, pubkey: &Pubkey) -> ClientResult<Account> {
                Ok(self
                    .get_account_responses
                    .borrow()
                    .get(pubkey)
                    .cloned()
                    .unwrap())
            }

            pub fn set_get_account_response(&self, pubkey: Pubkey, account: Account) {
                self.get_account_responses
                    .borrow_mut()
                    .insert(pubkey, account);
            }

            pub fn get_balance(&self, _pubkey: &Pubkey) -> ClientResult<u64> {
                Ok(0)
            }
        }
    }
}

pub mod solana_rpc_client_api {
    pub mod client_error {
        #[derive(thiserror::Error, Debug)]
        #[error("mock-error")]
        pub struct ClientError;
        pub type Result<T> = std::result::Result<T, ClientError>;
    }
}

pub mod solana_rpc_client_nonce_utils {
    use {
        super::super::nonce_inner::{
            state::{Data, DurableNonce},
            versions::Versions,
        },
        super::solana_sdk::{account::ReadableAccount, account_utils::StateMut, pubkey::Pubkey},
    };

    #[derive(thiserror::Error, Debug)]
    #[error("mock-error")]
    pub struct Error;

    pub fn data_from_account<T: ReadableAccount + StateMut<Versions>>(
        _account: &T,
    ) -> Result<Data, Error> {
        Ok(Data::new(
            Pubkey::new_unique(),
            DurableNonce::default(),
            5000,
        ))
    }
}

pub mod solana_account {
    use super::super::pubkey::Pubkey;
    #[derive(Clone)]
    pub struct Account {
        pub lamports: u64,
        pub data: Vec<u8>,
        pub owner: Pubkey,
        pub executable: bool,
    }

    pub trait ReadableAccount: Sized {
        fn data(&self) -> &[u8];
    }

    impl ReadableAccount for Account {
        fn data(&self) -> &[u8] {
            &self.data
        }
    }

    pub mod state_traits {
        use super::Account;

        pub trait StateMut<T> {}

        impl<T> StateMut<T> for Account {}
    }
}

pub mod solana_signature {
    #[derive(Default, Debug)]
    pub struct Signature;
}

pub mod solana_signer {
    use {super::super::pubkey::Pubkey, thiserror::Error};

    #[derive(Error, Debug)]
    #[error("mock-error")]
    pub struct SignerError;
    pub trait Signer {
        fn pubkey(&self) -> Pubkey;
    }

    pub mod signers {
        use super::Signer;

        pub trait Signers {}

        impl<T: Signer> Signers for [&T] {}
        impl<T: Signer> Signers for [&T; 1] {}
        impl<T: Signer> Signers for [&T; 2] {}
    }
}

pub mod solana_keypair {
    use {super::super::pubkey::Pubkey, super::solana_signer::Signer};
    pub struct Keypair;

    impl Keypair {
        pub fn new() -> Keypair {
            Keypair
        }
    }

    impl Signer for Keypair {
        fn pubkey(&self) -> Pubkey {
            Pubkey::default()
        }
    }
}

pub mod solana_transaction {
    use {
        super::super::hash_inner::Hash,
        super::super::instruction::Instruction,
        super::super::message_inner::Message,
        super::super::pubkey::Pubkey,
        super::solana_signer::{signers::Signers, SignerError},
        serde_derive::Serialize,
    };

    pub mod versioned {
        use {
            super::super::super::message_inner::VersionedMessage,
            super::super::{
                solana_signature::Signature,
                solana_signer::{signers::Signers, SignerError},
            },
        };
        pub struct VersionedTransaction {
            pub signatures: Vec<Signature>,
            pub message: VersionedMessage,
        }

        impl VersionedTransaction {
            pub fn try_new<T: Signers + ?Sized>(
                message: VersionedMessage,
                _keypairs: &T,
            ) -> std::result::Result<Self, SignerError> {
                Ok(VersionedTransaction {
                    signatures: vec![],
                    message,
                })
            }
        }
    }

    #[derive(Serialize)]
    pub struct Transaction {
        pub message: Message,
    }

    impl Transaction {
        pub fn new<T: Signers + ?Sized>(
            _from_keypairs: &T,
            _message: Message,
            _recent_blockhash: Hash,
        ) -> Transaction {
            Transaction {
                message: Message::new(&[], None),
            }
        }

        pub fn new_unsigned(_message: Message) -> Self {
            Transaction {
                message: Message::new(&[], None),
            }
        }

        pub fn new_with_payer(_instructions: &[Instruction], _payer: Option<&Pubkey>) -> Self {
            Transaction {
                message: Message::new(&[], None),
            }
        }

        pub fn new_signed_with_payer<T: Signers + ?Sized>(
            instructions: &[Instruction],
            payer: Option<&Pubkey>,
            signing_keypairs: &T,
            recent_blockhash: Hash,
        ) -> Self {
            let message = Message::new(instructions, payer);
            Self::new(signing_keypairs, message, recent_blockhash)
        }

        pub fn sign<T: Signers + ?Sized>(&mut self, _keypairs: &T, _recent_blockhash: Hash) {}

        pub fn try_sign<T: Signers + ?Sized>(
            &mut self,
            _keypairs: &T,
            _recent_blockhash: Hash,
        ) -> Result<(), SignerError> {
            Ok(())
        }
    }
}

/// Re-exports and mocks of solana-program modules that mirror those from
/// solana-program.
///
/// This lets examples in solana-program appear to be written as client
/// programs.
pub mod solana_sdk {
    pub use {
        super::super::clock_inner::Clock,
        super::super::hash_inner as hash,
        super::super::instruction,
        super::super::keccak_hasher_inner as keccak,
        super::super::message_inner as message,
        super::super::nonce_inner as nonce,
        super::super::pubkey::{self as pubkey, Pubkey},
        super::super::sdk_ids::{
            system_program,
            sysvar::{self, clock},
        },
        super::super::system_interface_inner::instruction as system_instruction,
        super::{
            solana_account::{self as account, state_traits as account_utils},
            solana_signer::{self as signer, signers},
        },
    };

    pub mod signature {
        pub use super::super::{
            solana_keypair::Keypair, solana_signature::Signature, solana_signer::Signer,
        };
    }

    pub mod transaction {
        pub use super::super::solana_transaction::{versioned::VersionedTransaction, Transaction};
    }

    pub mod address_lookup_table {
        pub use {
            super::super::super::address_lookup_table_interface_inner::{
                error, instruction, program, state,
            },
            super::super::super::message_inner::AddressLookupTableAccount,
        };
    }
}
