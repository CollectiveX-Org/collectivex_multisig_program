use crate::constants::{DISCRIMINATOR, MULTISIG_SEED, PROGRAM_CONFIG_SEED, TRANSACTION_SEED, PROPOSAL_SEED};
use crate::state::{Multisig, Proposal, ProposalStatus};
use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ProposalVote<'info> {
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

impl<'info> ProposalVote<'info> {
    /// Validates the vote action.
    pub fn validate(&self, vote: Vote) -> Result<()> {
        let Self {
            multisig,
            proposal,
            member,
            ..
        } = self;

        // Check if the signer is a valid member.
        require!(
            multisig.members.contains(&member.key()),
            ErrorCode::NotAMember
        );

        // Check if the member has the permission to vote.
        // require!(
        //     multisig.has_permission(member.key(), Permission::Vote),
        //     ErrorCode::Unauthorized
        // );

        // Validate based on vote type.
        match vote {
            Vote::Approve | Vote::Reject => {
                // Proposal must be active for approval/rejection.
                require!(
                    matches!(proposal.status, ProposalStatus::Active { .. }),
                    ErrorCode::InvalidProposalStatus
                );

                // Ensure the proposal is not stale.
                require!(
                    proposal.transaction_index > multisig.stale_transaction_index,
                    ErrorCode::StaleProposal
                );
            }
            Vote::Cancel => {
                // Proposal must be approved for cancellation.
                require!(
                    matches!(proposal.status, ProposalStatus::Approved { .. }),
                    ErrorCode::InvalidProposalStatus
                );
            }
        }

        Ok(())
    }

    /// Approves the proposal.
    pub fn proposal_approve(&mut self, _memo: Option<String>) -> Result<()> {
        let multisig = &mut self.multisig;
        let proposal = &mut self.proposal;
        let member = &self.member;

        // Approve the proposal.
        proposal.approve(member.key(), multisig.threshold as usize)?;

        Ok(())
    }

    pub fn proposal_reject(&mut self, _memo: Option<String>) -> Result<()> {
        let multisig = &mut self.multisig;
        let proposal = &mut self.proposal;
        let member = &self.member;

        let cutoff = multisig.members.len() - multisig.threshold as usize;
        // Approve the proposal.
        proposal.reject(member.key(), cutoff)?;

        Ok(())
    }
}

pub enum Vote {
    Approve,
    Reject,
    Cancel,
}
