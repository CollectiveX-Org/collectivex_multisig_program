use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::error::ErrorCode;
use crate::state::{ProgramConfig, Multisig, SpendingLimit};
use crate::constants::{DISCRIMINATOR, PROGRAM_CONFIG_SEED, MULTISIG_SEED, SPENDING_LIMIT_SEED};

#[derive(Accounts)]
pub struct MultisigAddSpendingLimit<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_CONFIG_SEED, MULTISIG_SEED, multisig.create_key.as_ref()],
        bump,
    )]
    pub multisig: Account<'info, Multisig>,

    // Multisig config authority
    #[account(mut)]
    pub config_authority: Signer<'info>,

    // used as a seed for deriving spending limit account
    pub create_key: Signer<'info>,

    /// The PDA for the SpendingLimit.
    #[account(
        init,
        payer = config_authority,
        space = DISCRIMINATOR + SpendingLimit::INIT_SPACE,
        seeds = [
            PROGRAM_CONFIG_SEED,
            multisig.key().as_ref(),
            SPENDING_LIMIT_SEED,
            create_key.key().as_ref()
        ],
        bump
    )]
    pub spending_limit: Account<'info, SpendingLimit>,

    pub system_program: Program<'info, System>,
}