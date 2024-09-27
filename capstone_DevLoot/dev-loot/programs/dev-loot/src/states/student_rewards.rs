use anchor_lang::prelude::*;

#[account]
pub struct StudentRewards{   
    pub diamond_student_rewards_ata: Pubkey, //TODO .. Change to Option <> [OPTIONAL] ?? 
    pub gold_student_rewards_ata: Pubkey, 
    //pub reward_mints: Pubkey,//TODO change to Vector RewardType to hold memecoins as well. Total 2 enums.
    pub reward_type: RewardType,//TODO make it optional 
    pub completed_at: i64, //date and time of completion.
    pub bump: u8
}

impl Space for StudentRewards {
    const INIT_SPACE: usize = 32 + 32 + (1 + 1) + 8 + 1; //TODO 1 + 1 is for Enum u8 + 1 discrimniator
}

//TWo separate instuctions for Diamond vs Gold with anchor
//or native code... 
//(zero_copy) to create bigger accounts and by pass Anchor
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum RewardType {
    NONE , 
    //TODO add memecoins later on
    //BONK, //memecoin for scores > 80% 
    DiamondNFT ,// for score > 80% (our own NFT)

    //POPCAT, // memecoin for scores < 80% 
    GoldNFT , // for scores < 80% (our own NFT)
}