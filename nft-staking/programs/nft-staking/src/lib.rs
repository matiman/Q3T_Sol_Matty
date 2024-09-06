use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

pub use instructions::*;

declare_id!("CQHcrauPJQHW6Fn6fhzgZ7yDPiFB5H4R1LiAM6yJyngG");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>,points_per_stake: u8,
        max_stake: u8,freeze_period: u32) -> Result<()> {
       
        ctx.accounts.initialize_config(points_per_stake, max_stake,freeze_period,&ctx.bumps)?;
        Ok(())
    }

    pub fn intialize_user(ctx: Context<InitializeUser>) -> Result<()> {

        ctx.accounts.initialize_user(&ctx.bumps)?;

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        
        ctx.accounts.stake(&ctx.bumps)?;

        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()?;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()?;

        Ok(())
    }

}
