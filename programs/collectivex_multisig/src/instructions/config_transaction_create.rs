use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::state::{Multisig, ConfigTransaction, ConfigAction};
use crate::constants::{DISCRIMINATOR, PROGRAM_CONFIG_SEED, MULTISIG_SEED, TRANSACTION_SEED};

#[derive(Accounts)]
#[instruction(actions: Vec<ConfigAction>)]
pub struct ConfigTransactionCreate<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_CONFIG_SEED, MULTISIG_SEED, multisig.create_key.as_ref()],
        bump,
    )]
    pub multisig: Account<'info, Multisig>,

    #[account(
        init,
        payer = rent_payer,
        space = ConfigTransaction::size(&actions),
        seeds = [
            PROGRAM_CONFIG_SEED,
            multisig.key().as_ref(),
            TRANSACTION_SEED,
            &multisig.transaction_index.to_le_bytes()
        ],
        bump
    )]
    pub transaction: Account<'info, ConfigTransaction>,

    pub creator: Signer<'info>,

    #[account(mut)]
    pub rent_payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ConfigTransactionCreate<'info> {
    /// Validates the inputs and signer for creating a config transaction.
    pub fn validate(&self, actions: &[ConfigAction]) -> Result<()> {
        // Ensure the creator is a member of the multisig.
        require!(
            self.multisig.members.contains(&self.creator.key()),
            ErrorCode::NotAMember
        );

        // Ensure at least one action is provided.
        require!(!actions.is_empty(), ErrorCode::NoActions);

        Ok(())
    }

    /// Creates the config transaction and updates the multisig.
    pub fn create_config_transaction(
        &mut self,
        actions: Vec<ConfigAction>,
        _memo: Option<String>,
    ) -> Result<()> {
        let multisig = &mut self.multisig;
        let transaction = &mut self.transaction;

        // Increment the transaction index.
        multisig.transaction_index = multisig.transaction_index.checked_add(1).unwrap();

        // Initialize the transaction fields.
        transaction.multisig = multisig.key();
        transaction.creator = self.creator.key();
        transaction.index = multisig.transaction_index;
        transaction.actions = actions;

        Ok(())
    }
}
