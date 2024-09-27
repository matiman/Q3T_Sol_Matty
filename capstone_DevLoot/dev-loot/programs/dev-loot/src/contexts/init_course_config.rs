use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::CourseConfig;

#[derive(Accounts)]
#[instruction(course_id: u8)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    //unique with course_id for each course
    #[account(
        init,
        payer = admin,
        space = 8 + CourseConfig::INIT_SPACE,
        seeds = [b"course_config".as_ref(),&[course_id]],
        bump
    )]
    pub course_config: Box<Account<'info,CourseConfig>>,

    // #[account(
    //     init,
    //     payer = admin,
    //     space = 8 + CorrectAnswers::INIT_SPACE,
    //     seeds = [b"correct_answers", course_config.key().as_ref() ],
    //     bump
    // )]
    // pub correct_answers: Box<Account<'info,CorrectAnswers>>,

    //create mint
    #[account(
        init,
        payer = admin,
        seeds = [b"gold_rewards_mint".as_ref(), course_config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = course_config,
    )]
    pub gold_rewards_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = admin,
        seeds = [b"diamond_rewards_mint".as_ref(), course_config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = course_config,
    )]
    pub diamond_rewards_mint: Box<InterfaceAccount<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,


}

impl<'info> InitConfig<'info> {

    pub fn initialize_config(&mut self, course_id: u8, last_content_index: u8,
         total_questions: u8,min_points_for_reward:u8, bumps: &InitConfigBumps) -> Result<()> { 

        self.course_config.set_inner(CourseConfig{
            course_id,
            last_content_index,
            total_questions: Some(total_questions),
            min_points_for_reward,
            diamond_rewards_mint: Some(self.diamond_rewards_mint.key()),
            gold_rewards_mint:Some(self.gold_rewards_mint.key()),

            bump: bumps.course_config,
            diamond_rewards_bump: Some(bumps.diamond_rewards_mint),
            gold_rewards_bump: bumps.gold_rewards_mint,
        });
        Ok(())
    }
    
}