use anchor_lang::prelude::*;
use crate::error::ErrorCode;

/// Tracks the status of a multisig proposal.
#[account]
pub struct Proposal {
    pub multisig: Pubkey,       // Associated multisig.
    pub transaction_index: u64, // Index of the transaction.
    pub status: ProposalStatus, // Status of the proposal.
    pub bump: u8,               // PDA bump seed.
    pub approved: Vec<Pubkey>,  // Approved members.
    pub rejected: Vec<Pubkey>,  // Rejected members.
    pub cancelled: Vec<Pubkey>, // Cancelled members.
}

impl Proposal {
    /// Calculate account size.
    pub fn size(members_len: usize) -> usize {
        8 +   // anchor account discriminator
        32 +  // multisig
        8 +   // index
        1 +   // status enum variant
        8 +   // status enum wrapped timestamp (i64)
        1 +   // bump
        (4 + (members_len * 32)) + // approved vec
        (4 + (members_len * 32)) + // rejected vec
        (4 + (members_len * 32)) // cancelled vec
    }

    /// Approve the proposal.
    pub fn approve(&mut self, member: Pubkey, threshold: usize) -> Result<()> {
        if !self.rejected.is_empty() {
            self.rejected.retain(|&x| x != member);
        }
        if !self.approved.contains(&member) {
            self.approved.push(member);
        }
        if self.approved.len() >= threshold {
            self.status = ProposalStatus::Approved;
        }
        Ok(())
    }

    /// Reject the proposal.
    pub fn reject(&mut self, member: Pubkey, cutoff: usize) -> Result<()> {
        if !self.approved.is_empty() {
            self.approved.retain(|&x| x != member);
        }
        if !self.rejected.contains(&member) {
            self.rejected.push(member);
        }
        if self.rejected.len() >= cutoff {
            self.status = ProposalStatus::Rejected;
        }
        Ok(())
    }

    /// Registers a cancellation vote.
    pub fn cancel(&mut self, member: Pubkey, threshold: usize) -> Result<()> {
        // Insert the vote of cancellation.
        match self.cancelled.binary_search(&member) {
            Ok(_) => return err!(ErrorCode::AlreadyCancelled),
            Err(pos) => self.cancelled.insert(pos, member),
        };

        // If current number of cancellations reaches threshold, mark the transaction as `Cancelled`.
        if self.cancelled.len() >= threshold {
            self.status = ProposalStatus::Cancelled;
        }

        Ok(())
    }
}

/// The status of a proposal.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum ProposalStatus {
    Draft,
    Active,
    Approved,
    Rejected,
    Cancelled
}
