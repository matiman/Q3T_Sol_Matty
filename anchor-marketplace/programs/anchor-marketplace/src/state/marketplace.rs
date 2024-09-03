use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin:Pubkey,
    pub sales_fee: u16,
    pub bump:u8,
    pub rewards_bump: u8,
    pub treasury_bump: u8,
    pub name: String
}

impl Space for Marketplace {
    // the 4 is for String and 32 is for 32 chars. 1 char 1 byte
    const INIT_SPACE: usize = 8 + 32 + 2 + 1 + 1 + 1 + (4+32); 
}