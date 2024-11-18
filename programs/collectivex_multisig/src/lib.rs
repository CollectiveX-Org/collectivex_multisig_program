use anchor_lang::prelude::*;
use anchor_lang::system_program;

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

    pub fn program_config_set_creation_fee(
        ctx: Context<ProgramConfigSetCreationFee>,
        new_creation_fee: u64,
    ) -> Result<()> {
        let program_config = &mut ctx.accounts.program_config;
    
        // Validate the current authority
        require_keys_eq!(
            program_config.authority,
            ctx.accounts.current_authority.key(),
            ErrorCode::InvalidAuthority
        );
    
        // Update the creation fee
        program_config.creation_fee = new_creation_fee;
    
        Ok(())
    }    

    pub fn program_config_set_treasury(
        ctx: Context<ProgramConfigSetTreasury>,
        new_treasury: Pubkey,
    ) -> Result<()> {
        let program_config = &mut ctx.accounts.program_config;
    
        // Validate the current authority
        require_keys_eq!(
            program_config.authority,
            ctx.accounts.current_authority.key(),
            ErrorCode::InvalidAuthority
        );
    
        // Ensure the new treasury address is valid
        require!(
            new_treasury != Pubkey::default(),
            ErrorCode::InvalidTreasury
        );
    
        // Update the treasury address
        program_config.treasury = new_treasury;
    
        Ok(())
    }    

    pub fn multisig_create(
        ctx: Context<MultisigCreate>,
        config_authority: Option<Pubkey>,
        threshold: u16,
        members: Vec<Pubkey>,
        time_lock: u32,
    ) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.treasury.key(),
            ctx.accounts.program_config.treasury,
            ErrorCode::InvalidTreasury
        );

        let multisig = &mut ctx.accounts.multisig;

        // Initialize the multisig account
        multisig.config_authority = config_authority.unwrap_or_default();
        multisig.threshold = threshold;
        multisig.time_lock = time_lock;
        multisig.members = members;

        // Transfer the creation fee to the treasury
        if ctx.accounts.program_config.creation_fee > 0 {
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.creator.to_account_info(),
                        to: ctx.accounts.treasury.to_account_info(),
                    },
                ),
                ctx.accounts.program_config.creation_fee,
            )?;
        }

        Ok(())
    }

    pub fn multisig_add_member(
        ctx: Context<MultisigAddMember>,
        new_member: Pubkey,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;

        // Validate the current authority
        require_keys_eq!(
            multisig.config_authority,
            ctx.accounts.config_authority.key(),
            ErrorCode::InvalidAuthority
        );
    
        // Ensure the new member is not already a part of the multisig
        require!(
            !multisig.members.contains(&new_member),
            ErrorCode::DuplicateMember
        );
    
        // Add the new member to the multisig and sort by key for consistency
        multisig.members.push(new_member);
        multisig.members.sort();
    
        // Ensure the maximum number of members isn't exceeded
        require!(
            multisig.members.len() <= 10,
            ErrorCode::ExceedsMaxMembers
        );

        Ok(())
    }    

    pub fn multisig_remove_member(
        ctx: Context<MultisigRemoveMember>,
        old_member: Pubkey,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;

        // Validate the current authority
        require_keys_eq!(
            multisig.config_authority,
            ctx.accounts.config_authority.key(),
            ErrorCode::InvalidAuthority
        );

        // Validate the current authority
        require_keys_eq!(
            multisig.config_authority,
            ctx.accounts.config_authority.key(),
            ErrorCode::InvalidAuthority
        );
    
        // Ensure there are enough members to perform multisig operations
        require!(
            multisig.members.len() > 1,
            ErrorCode::RemoveLastMember
        );
    
        let old_member_index = multisig
            .members
            .iter()
            .position(|&key| key == old_member)
            .ok_or(ErrorCode::NotAMember)?;

        // Remove the member from the list
        multisig.members.remove(old_member_index);

    
        Ok(())
    }    
    
    pub fn multisig_set_time_lock(
        ctx: Context<MultisigSetTimeLock>,
        new_time_lock: u32,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;

        // Validate the current authority
        require_keys_eq!(
            multisig.config_authority,
            ctx.accounts.config_authority.key(),
            ErrorCode::InvalidAuthority
        );
    
        // Update the time lock
        multisig.time_lock = new_time_lock;
    
        Ok(())
    }

    pub fn multisig_set_config_authority(
        ctx: Context<MultisigSetConfigAuthority>,
        new_config_authority: Pubkey,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;

        // Validate the current authority
        require_keys_eq!(
            multisig.config_authority,
            ctx.accounts.config_authority.key(),
            ErrorCode::InvalidAuthority
        );
    
        // Update the config authority
        multisig.config_authority = new_config_authority;

        Ok(())
    }    
}

#[derive(Accounts)]
pub struct ProgramConfigInit<'info> {
    #[account(
        init,
        payer = initializer,
        space = 8 + ProgramConfig::INIT_SPACE,
        seeds = [PROGRAM_CONFIG_SEED],
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
        seeds = [PROGRAM_CONFIG_SEED],
        bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub current_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ProgramConfigSetCreationFee<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_CONFIG_SEED],
        bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub current_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ProgramConfigSetTreasury<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_CONFIG_SEED],
        bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub current_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct MultisigCreate<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + Multisig::INIT_SPACE,
        seeds = [PROGRAM_CONFIG_SEED, MULTISIG_SEED, create_key.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, Multisig>,

    #[account(seeds = [PROGRAM_CONFIG_SEED], bump)]
    pub program_config: Account<'info, ProgramConfig>,

    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    // The account that will be used as the seed for the multisig PDA
    pub create_key: Signer<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MultisigAddMember<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_CONFIG_SEED, MULTISIG_SEED, multisig.create_key.as_ref()],
        bump,
    )]
    pub multisig: Account<'info, Multisig>,

    /// Authority of the current multisig to approve this operation
    #[account(mut)]
    pub config_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MultisigRemoveMember<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_CONFIG_SEED, MULTISIG_SEED, multisig.create_key.as_ref()],
        bump,
    )]
    pub multisig: Account<'info, Multisig>,

    /// Authority of the current multisig to approve this operation
    #[account(mut)]
    pub config_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MultisigSetTimeLock<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_CONFIG_SEED, MULTISIG_SEED, multisig.create_key.as_ref()],
        bump,
    )]
    pub multisig: Account<'info, Multisig>,

    /// Authority of the current multisig to approve this operation
    pub config_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct MultisigSetConfigAuthority<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_CONFIG_SEED, MULTISIG_SEED, multisig.create_key.as_ref()],
        bump,
    )]
    pub multisig: Account<'info, Multisig>,

    /// Authority of the current multisig to approve this operation
    pub config_authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Multisig {
    pub create_key: Pubkey,          // Key used as a seed to multisig PDA.
    pub config_authority: Pubkey,    // Authority to update the multisig
    pub threshold: u16,              // Number of signatures required
    #[max_len(10)]
    pub members: Vec<Pubkey>,       // Members of the multisig
    pub time_lock: u32,             // Time lock in seconds
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

pub const MULTISIG_SEED: &[u8] = b"multisig";
pub const PROGRAM_CONFIG_SEED: &[u8] = b"program_config";
