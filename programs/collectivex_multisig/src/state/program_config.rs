use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProgramConfig {
    pub authority: Pubkey, // Authority to update the config
    pub creation_fee: u64, // Multisig creation fee
    pub treasury: Pubkey,  // Treasury account for fees
}