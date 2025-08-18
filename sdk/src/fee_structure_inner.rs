//! Fee structures.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use std::num::NonZeroU32;

/// A fee and its associated compute unit limit
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct FeeBin {
    /// maximum compute units for which this fee will be charged
    pub limit: u64,
    /// fee in lamports
    pub fee: u64,
}

pub struct FeeBudgetLimits {
    pub loaded_accounts_data_size_limit: NonZeroU32,
    pub heap_cost: u64,
    pub compute_unit_limit: u64,
    pub prioritization_fee: u64,
}

/// Information used to calculate fees
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FeeStructure {
    /// lamports per signature
    pub lamports_per_signature: u64,
    /// lamports_per_write_lock
    pub lamports_per_write_lock: u64,
    /// Compute unit fee bins
    pub compute_fee_bins: Vec<FeeBin>,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct FeeDetails {
    transaction_fee: u64,
    prioritization_fee: u64,
}

impl FeeDetails {
    pub fn new(transaction_fee: u64, prioritization_fee: u64) -> Self {
        Self {
            transaction_fee,
            prioritization_fee,
        }
    }

    pub fn total_fee(&self) -> u64 {
        self.transaction_fee.saturating_add(self.prioritization_fee)
    }

    pub fn accumulate(&mut self, fee_details: &FeeDetails) {
        self.transaction_fee = self
            .transaction_fee
            .saturating_add(fee_details.transaction_fee);
        self.prioritization_fee = self
            .prioritization_fee
            .saturating_add(fee_details.prioritization_fee)
    }

    pub fn transaction_fee(&self) -> u64 {
        self.transaction_fee
    }

    pub fn prioritization_fee(&self) -> u64 {
        self.prioritization_fee
    }
}

pub const ACCOUNT_DATA_COST_PAGE_SIZE: u64 = 32_u64.saturating_mul(1024);

impl FeeStructure {
    pub fn get_max_fee(&self, num_signatures: u64, num_write_locks: u64) -> u64 {
        num_signatures
            .saturating_mul(self.lamports_per_signature)
            .saturating_add(num_write_locks.saturating_mul(self.lamports_per_write_lock))
            .saturating_add(
                self.compute_fee_bins
                    .last()
                    .map(|bin| bin.fee)
                    .unwrap_or_default(),
            )
    }

    pub fn calculate_memory_usage_cost(
        loaded_accounts_data_size_limit: u32,
        heap_cost: u64,
    ) -> u64 {
        (loaded_accounts_data_size_limit as u64)
            .saturating_add(ACCOUNT_DATA_COST_PAGE_SIZE.saturating_sub(1))
            .saturating_div(ACCOUNT_DATA_COST_PAGE_SIZE)
            .saturating_mul(heap_cost)
    }
}

impl Default for FeeStructure {
    fn default() -> Self {
        Self {
            lamports_per_signature: 5000,
            lamports_per_write_lock: 0,
            compute_fee_bins: vec![FeeBin {
                limit: 1_400_000,
                fee: 0,
            }],
        }
    }
}
