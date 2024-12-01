use anchor_lang::prelude::*;
use anchor_lang::solana_program::borsh0_10::get_instance_packed_len;

#[account]
pub struct ConfigTransaction {
    /// The multisig this transaction belongs to.
    pub multisig: Pubkey,
    /// The member of the multisig who created this transaction.
    pub creator: Pubkey,
    /// Unique index for this transaction in the context of the multisig.
    pub index: u64,
    /// Bump seed for the PDA of this transaction.
    pub bump: u8,
    /// Actions to be executed as part of this configuration transaction.
    pub actions: Vec<ConfigAction>,
}

impl ConfigTransaction {
    pub fn size(actions: &[ConfigAction]) -> usize {
        let actions_size: usize = actions
            .iter()
            .map(|action| get_instance_packed_len(action).unwrap())
            .sum();

        8 +   // discriminator
        32 +  // multisig
        32 +  // creator
        8 +   // index
        4 +  // actions length
        actions_size
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ConfigAction {
    AddMember { new_member: Pubkey },
    RemoveMember { old_member: Pubkey },
    ChangeThreshold { new_threshold: u16 },
    SetTimeLock { new_time_lock: u32 },
    AddSpendingLimit {
        create_key: Pubkey,
        vault_index: u8,
        mint: Pubkey,
        amount: u64,
        members: Vec<Pubkey>,
        destinations: Vec<Pubkey>,
    },
    RemoveSpendingLimit { spending_limit: Pubkey },
}
