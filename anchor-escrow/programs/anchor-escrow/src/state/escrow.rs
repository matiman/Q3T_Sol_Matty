use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    //In order to allow user to have multiple escrow accounts 
    pub seed: u64, 
    pub maker: Pubkey,
    //mint a is the one maker deposits/gives up in exchange for mint_b
    pub mint_a: Pubkey,
    //mint b is the mint taker want to take and give up mint_b
    pub mint_b: Pubkey,
    //We don't need the amount_to_send b/c at making, we send that amount to escrow/vault.
    pub amount_to_recieve: u64,
    pub bump: u8
}