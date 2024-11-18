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
}