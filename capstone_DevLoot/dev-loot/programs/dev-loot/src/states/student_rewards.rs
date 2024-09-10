use anchor_lang::prelude::*;

#[account]
pub struct StudentRewards{   
    pub student_ata: Pubkey, //TODO .. Change to Option <> [OPTIONAL] ?? 
    //Moved to Rewards Config
    //pub reward_mints: Pubkey,//TODO change to Vector RewardType to hold memecoins as well. Total 2 enums.
    pub completed_at: i64, //date and time of completion.
    pub bump: u8
}

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
// pub enum RewardType {
//     //TODO add memecoins later on
//     //BONK, //memecoin for scores > 80% 
//     DiamondNFT,// for score > 80% (our own NFT)

//     //POPCAT, // memecoin for scores < 80% 
//     GoldNFT, // for scores < 80% (our own NFT)
// }

impl Space for StudentRewards {
    const INIT_SPACE: usize = 32 + 8 + 1; //TODO 1 + 32 for Pubkey enum
}