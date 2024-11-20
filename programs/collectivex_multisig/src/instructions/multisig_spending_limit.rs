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

impl<'info> MultisigAddSpendingLimit<'info> {
    pub fn check_current_authority(&self) -> Result<()> {
        require_keys_eq!(
            self.multisig.config_authority,
            self.config_authority.key(),
            ErrorCode::InvalidAuthority
        );

        Ok(())
    }

    pub fn multisig_add_spending_limit(
        &mut self,
        bumps: &MultisigAddSpendingLimitBumps,
        vault_index: u8,
        mint: Pubkey,
        amount: u64,
        members: Vec<Pubkey>,
        destinations: Vec<Pubkey>,
    ) -> Result<()> {
        // Validate spending limit members size
        require!(
            members.len() <= 10,
            ErrorCode::ExceedsMaxMembers
        );

        // Initialize spending limit account
        self.spending_limit.multisig = self.multisig.key();
        self.spending_limit.create_key = self.create_key.key();
        self.spending_limit.vault_index = vault_index;
        self.spending_limit.mint = mint;
        self.spending_limit.amount = amount;
        self.spending_limit.remaining_amount = amount; // Start with full amount
        self.spending_limit.last_reset = Clock::get()?.unix_timestamp;
        self.spending_limit.bump = bumps.spending_limit;
        self.spending_limit.members = members;
        self.spending_limit.destinations = destinations;

        Ok(())
    }
}
