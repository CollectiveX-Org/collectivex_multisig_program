use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
/// Represents a multisig (multiple signature) configuration.
///
/// Fields:
/// - `create_key`: The key used as a seed to generate the multisig PDA (Program Derived Address).
/// - `config_authority`: The authority that has the permission to update the multisig configuration.
/// - `threshold`: The number of signatures required to approve a transaction.
/// - `members`: A vector containing the public keys of the members of the multisig. The maximum length is 10.
/// - `time_lock`: The time lock in seconds, which specifies the delay before a transaction can be executed.
/// - `transaction_index`: The index of the current transaction.
/// - `stale_transaction_index`: The index of the stale transaction.
pub struct Multisig {
    pub create_key: Pubkey,          // Key used as a seed to multisig PDA.
    pub config_authority: Pubkey,    // Authority to update the multisig
    pub threshold: u16,              // Number of signatures required
    #[max_len(10)]
    pub members: Vec<Pubkey>,       // Members of the multisig
    pub time_lock: u32,             // Time lock in seconds
    pub transaction_index: u64,     //The index of the current transaction.
    pub stale_transaction_index: u64,    // The index of the stale transaction.
}