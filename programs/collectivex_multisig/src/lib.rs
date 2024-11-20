pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::{multisig, program_config, spending_limit};

declare_id!("8bX4XyTtZH3xGRyE1Y4tEvhvmD4GHdjiXAsEMQ39ZUBy");

#[program]
pub mod collectivex_multisig {
    use super::*;

    pub fn program_config_init(
        ctx: Context<ProgramConfigInit>,
        authority: Pubkey,
        creation_fee: u64,
        treasury: Pubkey,
    ) -> Result<()> {
        ctx.accounts
            .init_program_config(authority, creation_fee, treasury)?;

        Ok(())
    }

    pub fn program_config_set_authority(
        ctx: Context<ProgramConfigEdit>,
        new_authority: Pubkey,
    ) -> Result<()> {
        ctx.accounts.check_current_authority()?;
        ctx.accounts.set_program_config_authority(new_authority)?;

        Ok(())
    }

    pub fn program_config_set_creation_fee(
        ctx: Context<ProgramConfigEdit>,
        new_creation_fee: u64,
    ) -> Result<()> {
        ctx.accounts.check_current_authority()?;
        ctx.accounts
            .set_program_config_creation_fee(new_creation_fee)?;

        Ok(())
    }

    pub fn program_config_set_treasury(
        ctx: Context<ProgramConfigEdit>,
        new_treasury: Pubkey,
    ) -> Result<()> {
        ctx.accounts.check_current_authority()?;
        ctx.accounts.set_program_config_treasury(new_treasury)?;

        Ok(())
    }

    pub fn multisig_create(
        ctx: Context<MultisigCreate>,
        config_authority: Pubkey,
        threshold: u16,
        members: Vec<Pubkey>,
        time_lock: u32,
    ) -> Result<()> {
        ctx.accounts
            .create_multisig(config_authority, threshold, members, time_lock)?;

        Ok(())
    }

    pub fn multisig_add_member(ctx: Context<MultisigEdit>, new_member: Pubkey) -> Result<()> {
        ctx.accounts.check_current_authority()?;
        ctx.accounts.add_multisig_member(new_member)?;

        Ok(())
    }

    pub fn multisig_remove_member(ctx: Context<MultisigEdit>, old_member: Pubkey) -> Result<()> {
        ctx.accounts.check_current_authority()?;
        ctx.accounts.remove_multisig_member(old_member)?;

        Ok(())
    }

    pub fn multisig_set_time_lock(ctx: Context<MultisigEdit>, new_time_lock: u32) -> Result<()> {
        ctx.accounts.check_current_authority()?;
        ctx.accounts.set_multisig_time_lock(new_time_lock)?;

        Ok(())
    }

    pub fn multisig_set_config_authority(
        ctx: Context<MultisigEdit>,
        new_config_authority: Pubkey,
    ) -> Result<()> {
        ctx.accounts.check_current_authority()?;
        ctx.accounts
            .set_multisig_config_authority(new_config_authority)?;

        Ok(())
    }

    pub fn multisig_add_spending_limit(
        ctx: Context<MultisigAddSpendingLimit>,
        vault_index: u8,
        mint: Pubkey,
        amount: u64,
        members: Vec<Pubkey>,
        destinations: Vec<Pubkey>,
    ) -> Result<()> {
        let _ = ctx.accounts.multisig_add_spending_limit(
            &ctx.bumps,
            vault_index,
            mint,
            amount,
            members,
            destinations,
        );

        Ok(())
    }
}
