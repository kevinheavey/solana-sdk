//! Re-exports the [`LamportsError`] type for backwards compatibility.
#[deprecated(
    since = "2.1.0",
    note = "Use solana_instruction_error::LamportsError instead"
)]
pub use solana_instruction_error::LamportsError;
