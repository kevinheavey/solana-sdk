//! Defines a transaction which supports multiple versions of messages.

#[cfg(feature = "bincode")]
use solana_signer::{signers::Signers, SignerError};
use {
    crate::Transaction,
    solana_message::{inline_nonce::is_advance_nonce_instruction_data, VersionedMessage},
    solana_sanitize::SanitizeError,
    solana_sdk_ids::system_program,
    solana_signature::Signature,
    std::cmp::Ordering,
};
#[cfg(feature = "serde")]
use {
    serde_derive::{Deserialize, Serialize},
    solana_short_vec as short_vec,
};

pub mod sanitized;

/// Type that serializes to the string "legacy"
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Legacy {
    Legacy,
}

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", untagged)
)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionVersion {
    Legacy(Legacy),
    Number(u8),
}

impl TransactionVersion {
    pub const LEGACY: Self = Self::Legacy(Legacy::Legacy);
}

// NOTE: Serialization-related changes must be paired with the direct read at sigverify.
/// An atomic transaction
#[cfg_attr(feature = "frozen-abi", derive(solana_frozen_abi_macro::AbiExample))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, PartialEq, Default, Eq, Clone)]
pub struct VersionedTransaction {
    /// List of signatures
    #[cfg_attr(feature = "serde", serde(with = "short_vec"))]
    pub signatures: Vec<Signature>,
    /// Message to sign.
    pub message: VersionedMessage,
}

impl From<Transaction> for VersionedTransaction {
    fn from(transaction: Transaction) -> Self {
        Self {
            signatures: transaction.signatures,
            message: VersionedMessage::Legacy(transaction.message),
        }
    }
}

impl VersionedTransaction {
    /// Signs a versioned message and if successful, returns a signed
    /// transaction.
    #[cfg(feature = "bincode")]
    pub fn try_new<T: Signers + ?Sized>(
        message: VersionedMessage,
        keypairs: &T,
    ) -> std::result::Result<Self, SignerError> {
        let static_account_keys = message.static_account_keys();
        if static_account_keys.len() < message.header().num_required_signatures as usize {
            return Err(SignerError::InvalidInput("invalid message".to_string()));
        }

        let signer_keys = keypairs.try_pubkeys()?;
        let expected_signer_keys =
            &static_account_keys[0..message.header().num_required_signatures as usize];

        match signer_keys.len().cmp(&expected_signer_keys.len()) {
            Ordering::Greater => Err(SignerError::TooManySigners),
            Ordering::Less => Err(SignerError::NotEnoughSigners),
            Ordering::Equal => Ok(()),
        }?;

        let message_data = message.serialize();
        let signature_indexes: Vec<usize> = expected_signer_keys
            .iter()
            .map(|signer_key| {
                signer_keys
                    .iter()
                    .position(|key| key == signer_key)
                    .ok_or(SignerError::KeypairPubkeyMismatch)
            })
            .collect::<std::result::Result<_, SignerError>>()?;

        let unordered_signatures = keypairs.try_sign_message(&message_data)?;
        let signatures: Vec<Signature> = signature_indexes
            .into_iter()
            .map(|index| {
                unordered_signatures
                    .get(index)
                    .copied()
                    .ok_or_else(|| SignerError::InvalidInput("invalid keypairs".to_string()))
            })
            .collect::<std::result::Result<_, SignerError>>()?;

        Ok(Self {
            signatures,
            message,
        })
    }

    pub fn sanitize(&self) -> std::result::Result<(), SanitizeError> {
        self.message.sanitize()?;
        self.sanitize_signatures()?;
        Ok(())
    }

    pub(crate) fn sanitize_signatures(&self) -> std::result::Result<(), SanitizeError> {
        Self::sanitize_signatures_inner(
            usize::from(self.message.header().num_required_signatures),
            self.message.static_account_keys().len(),
            self.signatures.len(),
        )
    }

    pub(crate) fn sanitize_signatures_inner(
        num_required_signatures: usize,
        num_static_account_keys: usize,
        num_signatures: usize,
    ) -> std::result::Result<(), SanitizeError> {
        match num_required_signatures.cmp(&num_signatures) {
            Ordering::Greater => Err(SanitizeError::IndexOutOfBounds),
            Ordering::Less => Err(SanitizeError::InvalidValue),
            Ordering::Equal => Ok(()),
        }?;

        // Signatures are verified before message keys are loaded so all signers
        // must correspond to static account keys.
        if num_signatures > num_static_account_keys {
            return Err(SanitizeError::IndexOutOfBounds);
        }

        Ok(())
    }

    /// Returns the version of the transaction
    pub fn version(&self) -> TransactionVersion {
        match self.message {
            VersionedMessage::Legacy(_) => TransactionVersion::LEGACY,
            VersionedMessage::V0(_) => TransactionVersion::Number(0),
        }
    }

    /// Returns a legacy transaction if the transaction message is legacy.
    pub fn into_legacy_transaction(self) -> Option<Transaction> {
        match self.message {
            VersionedMessage::Legacy(message) => Some(Transaction {
                signatures: self.signatures,
                message,
            }),
            _ => None,
        }
    }

    #[cfg(feature = "verify")]
    /// Verify the transaction and hash its message
    pub fn verify_and_hash_message(
        &self,
    ) -> solana_transaction_error::TransactionResult<solana_hash::Hash> {
        let message_bytes = self.message.serialize();
        if !self
            ._verify_with_results(&message_bytes)
            .iter()
            .all(|verify_result| *verify_result)
        {
            Err(solana_transaction_error::TransactionError::SignatureFailure)
        } else {
            Ok(VersionedMessage::hash_raw_message(&message_bytes))
        }
    }

    #[cfg(feature = "verify")]
    /// Verify the transaction and return a list of verification results
    pub fn verify_with_results(&self) -> Vec<bool> {
        let message_bytes = self.message.serialize();
        self._verify_with_results(&message_bytes)
    }

    #[cfg(feature = "verify")]
    fn _verify_with_results(&self, message_bytes: &[u8]) -> Vec<bool> {
        self.signatures
            .iter()
            .zip(self.message.static_account_keys().iter())
            .map(|(signature, pubkey)| signature.verify(pubkey.as_ref(), message_bytes))
            .collect()
    }

    /// Returns true if transaction begins with an advance nonce instruction.
    pub fn uses_durable_nonce(&self) -> bool {
        let message = &self.message;
        message
            .instructions()
            .get(crate::NONCED_TX_MARKER_IX_INDEX as usize)
            .filter(|instruction| {
                // Is system program
                matches!(
                    message.static_account_keys().get(instruction.program_id_index as usize),
                    Some(program_id) if system_program::check_id(program_id)
                ) && is_advance_nonce_instruction_data(&instruction.data)
            })
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        solana_hash::Hash,
        solana_instruction::{AccountMeta, Instruction},
        solana_keypair::Keypair,
        solana_message::Message as LegacyMessage,
        solana_pubkey::Pubkey,
        solana_signer::Signer,
        solana_system_interface::instruction as system_instruction,
    };

    #[test]
    fn test_try_new() {
        let keypair0 = Keypair::new();
        let keypair1 = Keypair::new();
        let keypair2 = Keypair::new();

        let message = VersionedMessage::Legacy(LegacyMessage::new(
            &[Instruction::new_with_bytes(
                Pubkey::new_unique(),
                &[],
                vec![
                    AccountMeta::new_readonly(keypair1.pubkey(), true),
                    AccountMeta::new_readonly(keypair2.pubkey(), false),
                ],
            )],
            Some(&keypair0.pubkey()),
        ));

        assert_eq!(
            VersionedTransaction::try_new(message.clone(), &[&keypair0]),
            Err(SignerError::NotEnoughSigners)
        );

        assert_eq!(
            VersionedTransaction::try_new(message.clone(), &[&keypair0, &keypair0]),
            Err(SignerError::KeypairPubkeyMismatch)
        );

        assert_eq!(
            VersionedTransaction::try_new(message.clone(), &[&keypair1, &keypair2]),
            Err(SignerError::KeypairPubkeyMismatch)
        );

        match VersionedTransaction::try_new(message.clone(), &[&keypair0, &keypair1]) {
            Ok(tx) => assert_eq!(tx.verify_with_results(), vec![true; 2]),
            Err(err) => assert_eq!(Some(err), None),
        }

        match VersionedTransaction::try_new(message, &[&keypair1, &keypair0]) {
            Ok(tx) => assert_eq!(tx.verify_with_results(), vec![true; 2]),
            Err(err) => assert_eq!(Some(err), None),
        }
    }

    fn nonced_transfer_tx() -> (Pubkey, Pubkey, VersionedTransaction) {
        let from_keypair = Keypair::new();
        let from_pubkey = from_keypair.pubkey();
        let nonce_keypair = Keypair::new();
        let nonce_pubkey = nonce_keypair.pubkey();
        let instructions = [
            system_instruction::advance_nonce_account(&nonce_pubkey, &nonce_pubkey),
            system_instruction::transfer(&from_pubkey, &nonce_pubkey, 42),
        ];
        let message = LegacyMessage::new(&instructions, Some(&nonce_pubkey));
        let tx = Transaction::new(&[&from_keypair, &nonce_keypair], message, Hash::default());
        (from_pubkey, nonce_pubkey, tx.into())
    }

    #[test]
    fn tx_uses_nonce_ok() {
        let (_, _, tx) = nonced_transfer_tx();
        assert!(tx.uses_durable_nonce());
    }

    #[test]
    fn tx_uses_nonce_empty_ix_fail() {
        assert!(!VersionedTransaction::default().uses_durable_nonce());
    }

    #[test]
    fn tx_uses_nonce_bad_prog_id_idx_fail() {
        let (_, _, mut tx) = nonced_transfer_tx();
        match &mut tx.message {
            VersionedMessage::Legacy(message) => {
                message.instructions.get_mut(0).unwrap().program_id_index = 255u8;
            }
            VersionedMessage::V0(_) => unreachable!(),
        };
        assert!(!tx.uses_durable_nonce());
    }

    #[test]
    fn tx_uses_nonce_first_prog_id_not_nonce_fail() {
        let from_keypair = Keypair::new();
        let from_pubkey = from_keypair.pubkey();
        let nonce_keypair = Keypair::new();
        let nonce_pubkey = nonce_keypair.pubkey();
        let instructions = [
            system_instruction::transfer(&from_pubkey, &nonce_pubkey, 42),
            system_instruction::advance_nonce_account(&nonce_pubkey, &nonce_pubkey),
        ];
        let message = LegacyMessage::new(&instructions, Some(&from_pubkey));
        let tx = Transaction::new(&[&from_keypair, &nonce_keypair], message, Hash::default());
        let tx = VersionedTransaction::from(tx);
        assert!(!tx.uses_durable_nonce());
    }

    #[test]
    fn tx_uses_nonce_wrong_first_nonce_ix_fail() {
        let from_keypair = Keypair::new();
        let from_pubkey = from_keypair.pubkey();
        let nonce_keypair = Keypair::new();
        let nonce_pubkey = nonce_keypair.pubkey();
        let instructions = [
            system_instruction::withdraw_nonce_account(
                &nonce_pubkey,
                &nonce_pubkey,
                &from_pubkey,
                42,
            ),
            system_instruction::transfer(&from_pubkey, &nonce_pubkey, 42),
        ];
        let message = LegacyMessage::new(&instructions, Some(&nonce_pubkey));
        let tx = Transaction::new(&[&from_keypair, &nonce_keypair], message, Hash::default());
        let tx = VersionedTransaction::from(tx);
        assert!(!tx.uses_durable_nonce());
    }

    #[test]
    fn test_sanitize_signatures_inner() {
        assert_eq!(
            VersionedTransaction::sanitize_signatures_inner(1, 1, 0),
            Err(SanitizeError::IndexOutOfBounds)
        );
        assert_eq!(
            VersionedTransaction::sanitize_signatures_inner(1, 1, 2),
            Err(SanitizeError::InvalidValue)
        );
        assert_eq!(
            VersionedTransaction::sanitize_signatures_inner(2, 1, 2),
            Err(SanitizeError::IndexOutOfBounds)
        );
        assert_eq!(
            VersionedTransaction::sanitize_signatures_inner(1, 1, 1),
            Ok(())
        );
    }
}
