use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config{
    pub rewards_mint: Pubkey, //TODO Make it enum and put it in Options
    pub bump: u8,
    pub rewards_bump: u8
}