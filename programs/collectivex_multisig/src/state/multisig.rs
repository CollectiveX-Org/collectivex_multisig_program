use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Multisig {
    pub create_key: Pubkey,          // Key used as a seed to multisig PDA.
    pub config_authority: Pubkey,    // Authority to update the multisig
    pub threshold: u16,              // Number of signatures required
    #[max_len(10)]
    pub members: Vec<Pubkey>,       // Members of the multisig
    pub time_lock: u32,             // Time lock in seconds
}