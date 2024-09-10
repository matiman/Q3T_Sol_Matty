use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, 
    token_interface::{Mint, MintTo, TokenAccount, TokenInterface, mint_to}};

use crate::{Config, Student, StudentError, StudentPrgress, StudentRewards};

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
        bump = config.bump
    )]
    pub config: Account<'info,Config>,

    #[account(
        mut,
        seeds = [b"rewards_mint".as_ref(), config.key().as_ref()],
        bump = config.rewards_bump
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = student,
        space = 8 + StudentRewards::INIT_SPACE,
        seeds = [b"student_rewards".as_ref(), student_account.wallet.as_str().as_ref()],
        bump,
    )]
    pub student_rewards: Account<'info, StudentRewards>,

    //The ata holds the acutal rewards_mint.
    #[account(
        init_if_needed,
        payer = student,
        associated_token::mint = rewards_mint,
        associated_token::authority = student //TODO should auth be student progress ?
    )]
    pub student_rewards_ata: InterfaceAccount<'info, TokenAccount>,

    //to fetch the wallet for progress seed derivation
    #[account(
        seeds = [b"student".as_ref(), student_account.wallet.as_str().as_ref()],
        bump = student_account.bump
    )]
    pub student_account: Account<'info,Student>,

    #[account(
        seeds = [b"student_progress".as_ref(), student_account.wallet.as_str().as_ref()],
        bump= student_progress.bump

    )]
    pub student_progress: Account<'info,StudentPrgress>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info,AssociatedToken>
}

impl<'info> RewardStudent<'info> {

    //TODO it should happen automatically when course is completed instead of users claiming a reward
    pub fn create_student_reward(&mut self) -> Result<()> {

        //only reward after course completion
        require!(self.student_progress.course_completed==true, StudentError::CourseNotCompleted);

        self.student_rewards.set_inner(StudentRewards{
            student_ata: self.student_rewards_ata.key(),
            completed_at: Clock::get()?.unix_timestamp,
            bump: self.config.rewards_bump,
        });

        Ok(())
    }

    pub fn reward_student(&mut self) -> Result<()> {

        //TODO only reward after course completion and reward is created.
        require!(self.student_progress.course_completed==true, StudentError::CourseNotCompleted);

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo{
            mint: self.rewards_mint.to_account_info(),
            to: self.student_rewards_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let seeds = &[
            b"config".as_ref(),
            &[self.config.bump]

        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_context, 1)?;

        Ok(())
    }

}