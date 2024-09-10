use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::Config;

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"config".as_ref()],
        bump
    )]
    pub config: Account<'info,Config>,

    //create mint
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards_mint".as_ref(), config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,


}

impl<'info> InitConfig<'info> {

    pub fn initialize_config(&mut self, bumps: &InitConfigBumps) -> Result<()> { 

        self.config.set_inner(Config{
            rewards_mint: self.rewards_mint.key(),
            bump: bumps.config,
            rewards_bump: bumps.rewards_mint,
        });
        Ok(())
    }
    
}