use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SpendingLimit {
    pub multisig: Pubkey,  // Multisig this spending limit belongs to.
    pub create_key: Pubkey,  // Key used to seed the SpendingLimit PDA.
    pub vault_index: u8,  // Index of the vault associated with this spending limit.
    pub mint: Pubkey,  // Token mint the spending limit is for.
    pub amount: u64, // Maximum amount of tokens allowed in a reset period.
    pub remaining_amount: u64, // Remaining tokens available for the current period.
    pub last_reset: i64, // Timestamp of the last reset (or creation).
    pub bump: u8, /// PDA bump for this SpendingLimit account.
    #[max_len(10)]
    pub members: Vec<Pubkey>, // Members of the multisig allowed to spend under this limit.
    #[max_len(10)]
    pub destinations: Vec<Pubkey>, // Allowed destinations for transfers under this spending limit.
}