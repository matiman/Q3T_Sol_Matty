use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

pub use instructions::*;

declare_id!("4EPq6SFvNwjPhZjCFfHStd5bw1ryGXM3d2cAj8ja9o5L");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
