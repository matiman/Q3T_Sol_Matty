use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::stake_config::StakeConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info>{
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = StakeConfig::INIT_SPACE,
        seeds = [b"config".as_ref()],
        bump,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = admin,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config
    )]
    pub rewards_mint: InterfaceAccount<'info,Mint>,

    pub system_program: Program<'info, System>,
    //for intializing mint rewards_mint
    pub token_program: Interface<'info, TokenInterface>,
    
}

impl<'info> InitializeConfig<'info> {

    pub fn initialize_config(&mut self, points_per_stake:u8, max_stake:u8,
         freeze_period:u32, bumps: &InitializeConfigBumps) -> Result<()>{

        self.config.set_inner(StakeConfig {
            points_per_stake,
            max_stake,
            freeze_period,
            bump: bumps.config,
            rewards_bump: bumps.rewards_mint
        });

        Ok(())
    }
    
}