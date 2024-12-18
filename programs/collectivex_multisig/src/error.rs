use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The authority provided is invalid.")]
    InvalidAuthority,
    #[msg("The treasury address is invalid.")]
    InvalidTreasury,
    #[msg("The member is already part of the multisig.")]
    DuplicateMember,
    #[msg("The multisig has reached its maximum member limit.")]
    ExceedsMaxMembers,
    #[msg("Cannot remove the last member from the multisig.")]
    RemoveLastMember,
    #[msg("The specified member is not part of the multisig.")]
    NotAMember,
    #[msg("Threshold exceeds the number of members.")]
    InvalidThreshold,
    #[msg("The specified spending limit is invalid.")]
    InvalidSpendingLimit,
    #[msg("No actions provided for the config transaction.")]
    NoActions,
    #[msg("Member already approved the transaction")]
    AlreadyApproved,
    #[msg("Member already rejected the transaction")]
    AlreadyRejected,
    #[msg("Member already cancelled the transaction")]
    AlreadyCancelled,
    #[msg("The signer does not have the required permissions.")]
    Unauthorized,
    #[msg("Proposal is in an invalid status.")]
    InvalidProposalStatus,
    #[msg("The proposal is stale.")]
    StaleProposal,
}