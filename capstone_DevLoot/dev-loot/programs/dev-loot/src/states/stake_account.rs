use anchor_lang::prelude::*;

#[account]
//Stake Account is per NFT owned by a user.
pub struct StakeAccount{
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub staked_at: i64,
    pub bump: u8

}

impl Space for StakeAccount {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}