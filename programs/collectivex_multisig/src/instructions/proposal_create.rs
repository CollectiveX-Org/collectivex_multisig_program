use crate::constants::{DISCRIMINATOR, MULTISIG_SEED, PROGRAM_CONFIG_SEED, TRANSACTION_SEED, PROPOSAL_SEED};
use crate::state::{Multisig, Proposal, ProposalStatus};
use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(transaction_index: u64)]
pub struct ProposalCreate<'info> {
    #[account(
        seeds = [PROGRAM_CONFIG_SEED, MULTISIG_SEED, multisig.create_key.as_ref()],
        bump,
    )]
    pub multisig: Account<'info, Multisig>,

    #[account(
        init,
        payer = rent_payer,
        space = Proposal::size(multisig.members.len()), // Example size, adjust as needed.
        seeds = [
            PROGRAM_CONFIG_SEED, 
            multisig.key().as_ref(), 
            TRANSACTION_SEED, 
            &transaction_index.to_le_bytes(), 
            PROPOSAL_SEED
        ],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    /// The member of the multisig that is creating the proposal.
    pub creator: Signer<'info>,
    #[account(mut)]
    pub rent_payer: Signer<'info>, // Rent payer for account creation.
    pub system_program: Program<'info, System>,
}

impl <'info> ProposalCreate<'info> {
    pub fn validate(&self) -> Result<()> {
        // Ensure the creator is a member of the multisig.
        require!(
            self.multisig.members.contains(&self.creator.key()),
            ErrorCode::NotAMember
        );
        Ok(())
    }

    pub fn proposal_create(
        &mut self,
        transaction_index: u64,
        draft: bool,
        bumps: &ProposalCreateBumps,
    ) -> Result<()> {
        let proposal = &mut self.proposal;
        proposal.multisig = self.multisig.key();
        proposal.transaction_index = transaction_index;
        proposal.status = if draft {
            ProposalStatus::Draft
        } else {
            ProposalStatus::Active
        };
        proposal.bump = bumps.proposal;
        proposal.approved = vec![];
        proposal.rejected = vec![];
        proposal.cancelled = vec![];
    
        Ok(())
    }    
}