use anchor_lang::prelude::*;

#[account]
pub struct StudentRewards{   
    pub student_ata: Pubkey, //[OPTIONAL] ?? 
    pub reward_mints: RewardType,//TODO change to Vector to hold memecoins as well. Total 2 enums.
    pub completed_at: i64, //date and time of completion.
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum RewardType {
    //TODO add memecoins later on
    //BONK, //memecoin for scores > 80% 
    DiamondNFT,// for score > 80% (our own NFT)

    //POPCAT, // memecoin for scores < 80% 
    GoldNFT, // for scores < 80% (our own NFT)
}

impl Space for StudentRewards {
    const INIT_SPACE: usize = 32 + (1 + 32)  + 8; // 1 + 32 for Pubkey enum
}