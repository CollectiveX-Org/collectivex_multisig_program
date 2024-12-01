use crate::constants::{DISCRIMINATOR, MULTISIG_SEED, PROGRAM_CONFIG_SEED, TRANSACTION_SEED, PROPOSAL_SEED};
use crate::state::{Multisig, Proposal, ProposalStatus};
use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ProposalActivate<'info> {
    #[account(
        seeds = [PROGRAM_CONFIG_SEED, MULTISIG_SEED, multisig.create_key.as_ref()],
        bump,
    )]
    pub multisig: Account<'info, Multisig>,

    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        mut,
        seeds = [
            PROGRAM_CONFIG_SEED,
            multisig.key().as_ref(),
            TRANSACTION_SEED,
            &proposal.transaction_index.to_le_bytes(),
            PROPOSAL_SEED,
        ],
        bump = proposal.bump,
    )]
    pub proposal: Account<'info, Proposal>,
}

impl<'info> ProposalActivate<'info> {
    /// Validates whether the proposal can be activated.
    pub fn validate(&self) -> Result<()> {
        let Self {
            multisig,
            proposal,
            member,
            ..
        } = self;

        // Check if the signer is a valid member of the multisig.
        require!(
            multisig.members.contains(&member.key()),
            ErrorCode::NotAMember
        );

        // // Check if the member has the necessary permissions.
        // require!(
        //     multisig.has_permission(member.key(), Permission::Initiate),
        //     ErrorCode::Unauthorized
        // );

        // Check if the proposal is in the `Draft` status.
        require!(
            matches!(proposal.status, ProposalStatus::Draft),
            ErrorCode::InvalidProposalStatus
        );

        // Ensure the proposal is not stale.
        require!(
            proposal.transaction_index > multisig.stale_transaction_index,
            ErrorCode::StaleProposal
        );

        Ok(())
    }

    /// Updates the status of the proposal to `Active`.
    pub fn proposal_activate(&mut self) -> Result<()> {
        self.proposal.status = ProposalStatus::Active;

        Ok(())
    }
}
