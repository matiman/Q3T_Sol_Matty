use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub points_earned:i64,
    pub amount_staked: u8,
    pub bump: u8
}

impl Space for UserAccount {
    const INIT_SPACE: usize = 8 + 4 + 1 + 1;
}