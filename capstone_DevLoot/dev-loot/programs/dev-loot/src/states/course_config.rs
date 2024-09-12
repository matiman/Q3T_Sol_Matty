use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CourseConfig{
    pub course_id: u8, //each config is per course.
    pub last_content_index: u8,
    pub total_questions: Option<u8>,
    pub min_points_for_reward: u8,

    pub diamond_rewards_mint: Option<Pubkey>, //TODO Make it enum and put it in Options
    pub gold_rewards_mint: Option<Pubkey>,

    pub bump: u8,
    pub diamond_rewards_bump: Option<u8>,
    pub gold_rewards_bump: u8
}

