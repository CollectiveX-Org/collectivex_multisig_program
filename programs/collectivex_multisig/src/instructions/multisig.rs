use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::error::ErrorCode;
use crate::state::{ProgramConfig, Multisig};
use crate::constants::{DISCRIMINATOR, PROGRAM_CONFIG_SEED, MULTISIG_SEED};

#[derive(Accounts)]
pub struct MultisigCreate<'info> {
    #[account(
        init,
        payer = creator,
        space = DISCRIMINATOR + Multisig::INIT_SPACE,
        seeds = [PROGRAM_CONFIG_SEED, MULTISIG_SEED, create_key.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, Multisig>,

    #[account(seeds = [PROGRAM_CONFIG_SEED], bump)]
    pub program_config: Account<'info, ProgramConfig>,

    /// CHECK: for creation fee
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    // used as a seed for deriving multisig account
    pub create_key: Signer<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl <'info> MultisigCreate<'info> {
    pub fn create_multisig(
        &mut self,
        config_authority: Pubkey,
        threshold: u16,
        members: Vec<Pubkey>,
        time_lock: u32
    ) -> Result<()> {
        require_keys_eq!(
            self.treasury.key(),
            self.program_config.treasury,
            ErrorCode::InvalidTreasury
        );

        self.multisig.config_authority = config_authority;
        self.multisig.threshold = threshold;
        self.multisig.time_lock = time_lock;
        self.multisig.members = members;

        // Transfer the creation fee to the treasury
        if self.program_config.creation_fee > 0 {
            let cpi_accounts = Transfer {
                from: self.creator.to_account_info(),
                to: self.treasury.to_account_info(),
            };

            let cpi_program = self.system_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            transfer(cpi_ctx, self.program_config.creation_fee)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MultisigEdit<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_CONFIG_SEED, MULTISIG_SEED, multisig.create_key.as_ref()],
        bump,
    )]
    pub multisig: Account<'info, Multisig>,

    #[account(mut)]
    pub config_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl <'info> MultisigEdit<'info> {
    pub fn check_current_authority(&self) -> Result<()> {
        require_keys_eq!(
            self.multisig.config_authority,
            self.config_authority.key(),
            ErrorCode::InvalidAuthority
        );

        Ok(())
    }

    pub fn add_multisig_member(
        &mut self,
        new_member: Pubkey
    ) -> Result<()> {
        require!(
            !self.multisig.members.contains(&new_member),
            ErrorCode::DuplicateMember
        );

        require!(
            self.multisig.members.len() > 9,
            ErrorCode::ExceedsMaxMembers
        );

        self.multisig.members.push(new_member);

        Ok(())
    }

    pub fn remove_multisig_member(
        &mut self,
        old_member: Pubkey
    ) -> Result<()> {
        let index = self.multisig
            .members
            .iter()
            .position(|&x| x == old_member)
            .ok_or(ErrorCode::NotAMember)?;

        self.multisig.members.remove(index);

        Ok(())
    }

    pub fn set_multisig_time_lock(
        &mut self,
        new_time_lock: u32
    ) -> Result<()> {
        self.multisig.time_lock = new_time_lock;

        Ok(())
    }

    pub fn set_multisig_config_authority(
        &mut self,
        new_config_authority: Pubkey
    ) -> Result<()> {
        self.multisig.config_authority = new_config_authority;

        Ok(())
    }
}