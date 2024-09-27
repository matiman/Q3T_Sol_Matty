use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, 
    token_interface::{Mint, MintTo, TokenAccount, TokenInterface, mint_to}};

use crate::{CourseConfig, Student, DevLootErrorCodes, StudentPrgress, StudentRewards , RewardType};

#[derive(Accounts)]
pub struct RewardStudent<'info> {
    //TODO 
    //Just for test only
    //Signer should be changed to SystemAccount and only do CPI call to claim after course completion.
    //config should be the PDA that signs the tx. Student just pays for it.

    #[account(mut)]
    pub student: Signer<'info>, 

    #[account(
        seeds = [b"config".as_ref()],
        bump = course_config.bump
    )]
    pub course_config: Box<Account<'info,CourseConfig>>,

    #[account(
        mut,
        seeds = [b"diamond_rewards_mint".as_ref(), course_config.key().as_ref()],
        bump = course_config.diamond_rewards_bump.unwrap_or_default()
    )]
    pub diamond_rewards_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        seeds = [b"gold_rewards_mint".as_ref(), course_config.key().as_ref()],
        bump = course_config.gold_rewards_bump
    )]
    pub gold_rewards_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = student,
        space = 8 + StudentRewards::INIT_SPACE,
        seeds = [b"student_rewards".as_ref(), student_account.wallet.key().as_ref()],
        bump,
    )]
    pub student_rewards: Box<Account<'info, StudentRewards>>,

    //TODO Student should only have gold or diamond ata. Not both.
    //The ata holds the acutal rewards_mint.
    #[account(
        init_if_needed,
        payer = student,
        associated_token::mint = diamond_rewards_mint,
        associated_token::authority = student //TODO should auth be student progress ?
    )]
    pub student_diamond_rewards_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    //The ata holds the acutal rewards_mint.
    #[account(
        init_if_needed,
        payer = student,
        associated_token::mint = gold_rewards_mint,
        associated_token::authority = student //TODO should auth be student progress ?
    )]
    pub student_gold_rewards_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    //to fetch the wallet for progress seed derivation
    #[account(
        seeds = [b"student".as_ref(), student_account.wallet.key().as_ref()],
        bump = student_account.bump
    )]
    pub student_account: Box<Account<'info,Student>>,

    #[account(
        seeds = [b"student_progress".as_ref(), student_account.wallet.key().as_ref()],
        bump= student_progress.bump

    )]
    pub student_progress: Box<Account<'info,StudentPrgress>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info,AssociatedToken>
}

impl<'info> RewardStudent<'info> {

    //TODO it should happen automatically when course is completed instead of users claiming a reward
    pub fn create_student_reward(&mut self, bumps: &RewardStudentBumps) -> Result<()> {

        //only reward after course completion
        require!(self.student_progress.course_completed==true, DevLootErrorCodes::CourseNotCompleted);
    
        self.student_rewards.set_inner(StudentRewards{
            diamond_student_rewards_ata: self.student_diamond_rewards_ata.key(),
            gold_student_rewards_ata: self.student_gold_rewards_ata.key(),
            //Initiation with no reward
            reward_type: RewardType::NONE,
            completed_at: Clock::get()?.unix_timestamp,
            bump: bumps.student_rewards,
            
        });

        Ok(())
    }

    pub fn reward_student(&mut self) -> Result<()> {

        //TODO only reward after course completion and reward is created.
        require!(self.student_progress.course_completed==true, DevLootErrorCodes::CourseNotCompleted);

        let is_diamond_student = self.student_progress.total_points_earned >= self.course_config.min_points_for_reward;
    
        self.reward_dev_loot_nft(is_diamond_student)?;

        //dimanod student recieves memecoins if they are paid students 
        if is_diamond_student && self.student_account.is_paid_student {
            self.reward_memecoins()?;
        }

        Ok(())
    }

    fn reward_dev_loot_nft(&mut self, is_diamond_student: bool) -> Result<()>{

        let (mint, to, reward_type) = 
        match is_diamond_student {
            true => {(
                self.diamond_rewards_mint.to_account_info(),
                self.student_diamond_rewards_ata.to_account_info(),
                RewardType::DiamondNFT,
            )},
            false =>  {(
                 self.gold_rewards_mint.to_account_info(),
                self.student_gold_rewards_ata.to_account_info(),
                RewardType::GoldNFT
            )}
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo{
            mint,
            to,
            authority: self.course_config.to_account_info(),
        };

        let seeds = &[
            b"config".as_ref(),
            &[self.course_config.bump]

        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        //mint the right NFT to
        mint_to(cpi_context, 1)?;

        //update student reward type
        self.student_rewards.reward_type = reward_type;

        Ok(())

    }

    //TODO do it via typescript isntead ?? 
    fn reward_memecoins(&mut self) -> Result<()>{

        Ok(())

    }

}