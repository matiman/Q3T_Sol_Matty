use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::states::stake_config::StakeConfig;

#[derive(Accounts)]
pub struct InitializeStakingConfig<'info>{
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = StakeConfig::INIT_SPACE,
        seeds = [b"stake_config".as_ref()],
        bump,
    )]
    pub stake_config: Account<'info, StakeConfig>,

    //reward given to students for staking their diamon mint (only for diamond students after they complete course)
    #[account(
        init,
        payer = admin,
        seeds = [b"stake_rewards".as_ref(), stake_config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = stake_config
    )]
    pub stake_rewards_mint: InterfaceAccount<'info,Mint>,

    pub system_program: Program<'info, System>,
    //for intializing mint rewards_mint
    pub token_program: Interface<'info, TokenInterface>,
    
}

impl<'info> InitializeStakingConfig<'info> {

    pub fn initialize_config(&mut self, points_per_stake:u8, max_stake:u8,
         freeze_period:u32, bumps: &InitializeStakingConfigBumps) -> Result<()>{

        self.stake_config.set_inner(StakeConfig {
            points_per_stake,
            max_stake,
            freeze_period,
            bump: bumps.stake_config,
            rewards_bump: bumps.stake_rewards_mint
        });

        Ok(())
    }
    
}