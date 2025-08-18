pub use super::sysvar_id_inner::SysvarId;
#[deprecated(since = "2.1.0", note = "Use `solana-sysvar-id` crate instead")]
pub use crate::{declare_deprecated_sysvar_id, declare_sysvar_id};
#[deprecated(since = "2.2.0", note = "Use `solana-sysvar` crate instead")]
#[allow(deprecated)]
pub use {
    super::sdk_ids::sysvar::{check_id, id, ID},
    super::sysvar_inner::{
        clock, epoch_rewards, epoch_schedule, fees, last_restart_slot, recent_blockhashes, rent,
        rewards, slot_hashes, slot_history, Sysvar, SysvarSerialize,
    },
};

pub mod instructions {
    #[deprecated(since = "2.2.0", note = "Use solana-instruction crate instead")]
    pub use super::super::instruction_inner::{BorrowedAccountMeta, BorrowedInstruction};
    #[cfg(not(target_os = "solana"))]
    #[deprecated(since = "2.2.0", note = "Use solana-instructions-sysvar crate instead")]
    pub use super::super::instructions_sysvar_inner::construct_instructions_data;
    #[deprecated(since = "2.2.0", note = "Use solana-instructions-sysvar crate instead")]
    #[allow(deprecated)]
    pub use super::super::instructions_sysvar_inner::{
        get_instruction_relative, load_current_index_checked, load_instruction_at_checked,
        Instructions,
    };
    #[deprecated(since = "2.2.0", note = "Use solana-sdk-ids crate instead")]
    pub use super::super::sdk_ids::sysvar::instructions::{check_id, id, ID};
}
