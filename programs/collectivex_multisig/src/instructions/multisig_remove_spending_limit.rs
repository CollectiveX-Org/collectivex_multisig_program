use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::state::{Multisig, SpendingLimit};
use crate::constants::{PROGRAM_CONFIG_SEED, MULTISIG_SEED, SPENDING_LIMIT_SEED};

#[derive(Accounts)]
pub struct MultisigRemoveSpendingLimit<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_CONFIG_SEED, MULTISIG_SEED, multisig.create_key.as_ref()],
        bump,
    )]
    pub multisig: Account<'info, Multisig>,

    /// Config authority that must authorize the removal of the spending limit.
    #[account(mut)]
    pub config_authority: Signer<'info>,

    /// SpendingLimit to be removed.
    #[account(
        mut,
        close = rent_collector,
        constraint = spending_limit.multisig == multisig.key() @ ErrorCode::InvalidSpendingLimit
    )]
    pub spending_limit: Account<'info, SpendingLimit>,

    /// Rent collector to collect lamports upon closing the SpendingLimit account.
    /// CHECK: This account will receive the rent.
    #[account(mut)]
    pub rent_collector: AccountInfo<'info>,
}

impl<'info> MultisigRemoveSpendingLimit<'info> {
    /// Validates the authority of the current signer.
    pub fn check_current_authority(&self) -> Result<()> {
        require_keys_eq!(
            self.multisig.config_authority,
            self.config_authority.key(),
            ErrorCode::InvalidAuthority
        );
        Ok(())
    }

    pub fn multisig_remove_spending_limit(
        &mut self,
        _memo: Option<String>,
    ) -> Result<()> {
        msg!("Spending limit removed successfully.");
        Ok(())
    }
}
