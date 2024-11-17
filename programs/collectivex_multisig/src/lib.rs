use anchor_lang::prelude::*;

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
        // Initialize the program configuration
        let program_config = &mut ctx.accounts.program_config;

        // Set values from the arguments
        program_config.authority = authority;
        program_config.creation_fee = creation_fee;
        program_config.treasury = treasury;

        // Ensure validity of the configuration
        require!(
            program_config.authority != Pubkey::default(),
            ErrorCode::InvalidAuthority
        );
        require!(
            program_config.treasury != Pubkey::default(),
            ErrorCode::InvalidTreasury
        );

        Ok(())
    }

    pub fn program_config_set_authority(
        ctx: Context<ProgramConfigSetAuthority>,
        new_authority: Pubkey,
    ) -> Result<()> {
        let program_config = &mut ctx.accounts.program_config;
    
        // Validate the current authority
        require_keys_eq!(
            program_config.authority,
            ctx.accounts.current_authority.key(),
            ErrorCode::InvalidAuthority
        );
    
        // Update the authority
        program_config.authority = new_authority;
    
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct ProgramConfigInit<'info> {
    #[account(
        init,
        payer = initializer,
        space = 8 + ProgramConfig::INIT_SPACE,
        seeds = [MULTISIG_SEED, PROGRAM_CONFIG_SEED],
        bump
    )]
    pub program_config: Account<'info, ProgramConfig>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProgramConfigSetAuthority<'info> {
    #[account(
        mut,
        seeds = [MULTISIG_SEED, PROGRAM_CONFIG_SEED],
        bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub current_authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct ProgramConfig {
    pub authority: Pubkey, // Authority to update the config
    pub creation_fee: u64, // Multisig creation fee
    pub treasury: Pubkey,  // Treasury account for fees
}

#[error_code]
pub enum ErrorCode {
    #[msg("The authority provided is invalid.")]
    InvalidAuthority,
    #[msg("The treasury address is invalid.")]
    InvalidTreasury,
}

pub const MULTISIG_SEED: &[u8] = b"multisig";
pub const PROGRAM_CONFIG_SEED: &[u8] = b"program_config";
