
use anchor_lang::prelude::*;
use anchor_spl::{metadata::{mpl_token_metadata::instructions::{
                ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata},
                token::{revoke, Revoke}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{states::{stake_account::StakeAccount, stake_config::StakeConfig}, CourseConfig, StudentPrgress};
use crate::errors::DevLootErrorCodes;
#[derive(Accounts)]
pub struct UnstakeDiamondNft<'info> {
    #[account(mut)]
    pub student: Signer<'info>,

    #[account(
        seeds = [b"course_config".as_ref(),&[course_config.course_id]],
        bump =course_config.bump
    )]
    pub course_config: Box<Account<'info,CourseConfig>>,

    //the NFT the diamond user is about to stake
    #[account(
        mut,
        seeds = [b"diamond_rewards_mint".as_ref(), course_config.key().as_ref()],
        bump= course_config.diamond_rewards_bump.unwrap(),
    )]
    pub diamond_rewards_mint: Box<InterfaceAccount<'info, Mint>>,

    //user acct to update points earned to some amt and staked to 0 when unstaking
    #[account(
        mut,
        seeds = [b"student_progress".as_ref(), student.key().as_ref()],
        bump
    )]
    pub student_progress: Account<'info, StudentPrgress>,

    //user_token_ata
    #[account(
        mut,
        associated_token::mint = diamond_rewards_mint,
        associated_token::authority = student
    )]
    pub student_token_ata: InterfaceAccount<'info, TokenAccount>,

    //config .. just to read config. Not writing to it.
    #[account(
        seeds = [b"stake_config".as_ref()],
        bump = stake_config.bump
    )]
    pub stake_config: Account<'info,StakeConfig>,

    //stake_account
    #[account(
        mut,
        close = student,
        seeds =[b"stake_account".as_ref(), diamond_rewards_mint.key().as_ref(),stake_config.key().as_ref()],
        bump = stake_account.bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    //metadata and collection isn't needed here since verification has been done when staking.

    //edition
    #[account (
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            diamond_rewards_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
   
    pub edition: Account<'info,MasterEditionAccount>,

    //token program
    pub token_program: Interface<'info,TokenInterface>,
    //metadata program
    pub metadata_program: Program<'info, Metadata>,
    //system program
    pub system_program: Program<'info, System>
}

impl <'info> UnstakeDiamondNft<'info>{

    pub fn unstake(&mut self) -> Result<()> { 

        //check if freeze time passed
        let staked_time = self.stake_account.staked_at;
        let current_time = Clock::get()?.unix_timestamp;
        //change seconds to days.
        let time_elapsed = ((current_time - staked_time) / 86400) as u32;

        require!(time_elapsed >= self.stake_config.freeze_period, DevLootErrorCodes::FreezePeriodNotPassed);

        let seeds = &[
            b"stake_account",
            self.diamond_rewards_mint.to_account_info().key.as_ref(),
            self.stake_config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];     
        let signer_seeds = &[&seeds[..]];

            let metadata_program = &self.metadata_program.to_account_info();
            let delegate = &self.stake_account.to_account_info();
            let token_account = &self.student_token_ata.to_account_info();
            let edition = &self.edition.to_account_info();
            let mint = &self.diamond_rewards_mint.to_account_info();
            let token_program = &self.token_program.to_account_info();

            ThawDelegatedAccountCpi::new(metadata_program, 
                ThawDelegatedAccountCpiAccounts{
                    delegate,
                    token_account,
                    edition,
                    mint,
                    token_program
            }).invoke_signed(signer_seeds)?;

            let cpi_program = self.token_program.to_account_info();

            //revoke mint authority from stake account
            let cpi_accounts = Revoke{
                source: self.student_token_ata.to_account_info(),
                authority: self.student.to_account_info()
            };

            let cpi_context =  CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            revoke(cpi_context)?;
            
            //decrement amount staked by 1
            self.student_progress.amount_staked =self.student_progress.amount_staked.checked_sub(1).ok_or(ProgramError::ArithmeticOverflow)?;
        

        Ok(())
    }

}