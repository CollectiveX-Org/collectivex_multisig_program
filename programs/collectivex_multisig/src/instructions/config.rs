use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::state::ProgramConfig;
use crate::constants::{DISCRIMINATOR, PROGRAM_CONFIG_SEED};

#[derive(Accounts)]
pub struct ProgramConfigInit<'info> {
    #[account(
        init,
        payer = initializer,
        space = DISCRIMINATOR + ProgramConfig::INIT_SPACE,
        seeds = [PROGRAM_CONFIG_SEED],
        bump
    )]
    pub program_config: Account<'info, ProgramConfig>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl <'info> ProgramConfigInit<'info> {
    pub fn init_program_config(
        &mut self,
        authority: Pubkey,
        creation_fee: u64,
        treasury: Pubkey
    ) -> Result<()> {
        require!(
            self.program_config.authority != Pubkey::default(),
            ErrorCode::InvalidAuthority
        );

        require!(
            self.program_config.treasury != Pubkey::default(),
            ErrorCode::InvalidTreasury
        );

        self.program_config.set_inner(ProgramConfig {
            authority,
            creation_fee,
            treasury
        });
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProgramConfigEdit<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_CONFIG_SEED],
        bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub current_authority: Signer<'info>,
}

impl <'info> ProgramConfigEdit<'info> {
    pub fn check_current_authority(&self) -> Result<()> {
        require_keys_eq!(
            self.program_config.authority,
            self.current_authority.key(),
            ErrorCode::InvalidAuthority
        );

        Ok(())
    }

    pub fn set_program_config_authority(
        &mut self,
        new_authority: Pubkey
    ) -> Result<()> {
        self.program_config.authority = new_authority;

        Ok(())
    }

    pub fn set_program_config_creation_fee(
        &mut self,
        new_creation_fee: u64
    ) -> Result<()> {
        self.program_config.creation_fee = new_creation_fee;

        Ok(())
    }

    pub fn set_program_config_treasury(
        &mut self,
        new_treasury: Pubkey
    ) -> Result<()> {
        require!(
            new_treasury != Pubkey::default(),
            ErrorCode::InvalidTreasury
        );

        self.program_config.treasury = new_treasury;

        Ok(())
    }
}