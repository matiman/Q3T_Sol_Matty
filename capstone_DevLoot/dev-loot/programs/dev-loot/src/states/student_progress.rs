use anchor_lang::prelude::*;

#[account]
pub struct StudentPrgress{
    pub course_id: u8, // the Pubkey of the course. [OPTIONAL]
    pub content_at: u8, // to store the index of the content the student is at starting from 0
    pub total_points_earned: u8, // total questions answered by the student
    pub course_completed: bool, //[OPTIONAL] if student completed the course to save on computation.
    pub last_updated: i64, //last progress update 
    pub registered_at: i64,
    pub staking_points_earned: i64,//when student stakes diamond NFT after course completion
    pub amount_staked: u8,
    pub bump: u8,
}

impl Space for StudentPrgress{
    const INIT_SPACE: usize = 1 + 1 + 1 + 1 + 8 + 8 + 1 + 8 + 1;
}