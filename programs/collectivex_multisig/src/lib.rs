use anchor_lang::prelude::*;

declare_id!("8bX4XyTtZH3xGRyE1Y4tEvhvmD4GHdjiXAsEMQ39ZUBy");

#[program]
pub mod collectivex_multisig {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
