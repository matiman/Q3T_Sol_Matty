
use anchor_lang::prelude::*;
use anchor_spl::{metadata::{mpl_token_metadata::instructions::{
                ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata},
                token::{revoke, Revoke}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::{stake_account::StakeAccount, stake_config::StakeConfig, user_account::UserAccount};
use crate::errors::StakingError;
#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    //mint
    pub mint: InterfaceAccount<'info,Mint>,

    //user acct to update points earned to some amt and staked to 0 when unstaking
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    //user_token_ata
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_token_ata: InterfaceAccount<'info, TokenAccount>,

    //config .. just to read config. Not writing to it.
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info,StakeConfig>,

    //stake_account
    #[account(
        mut,
        close = user,
        seeds =[b"stake".as_ref(), mint.key().as_ref(),config.key().as_ref()],
        bump = stake_account.bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    //metadata and collection isn't needed here since verification has been done when staking.

    //edition
    #[account (
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
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

impl <'info> Unstake<'info>{

    pub fn unstake(&mut self) -> Result<()> { 

        //check if freeze time passed
        let staked_time = self.stake_account.staked_at;
        let current_time = Clock::get()?.unix_timestamp;
        //change seconds to days.
        let time_elapsed = ((current_time - staked_time) / 86400) as u32;

        require!(time_elapsed >= self.config.freeze_period, StakingError::FreezePeriodNotPassed);

        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];     
        let signer_seeds = &[&seeds[..]];

            let metadata_program = &self.metadata_program.to_account_info();
            let delegate = &self.stake_account.to_account_info();
            let token_account = &self.user_token_ata.to_account_info();
            let edition = &self.edition.to_account_info();
            let mint = &self.mint.to_account_info();
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
                source: self.user_token_ata.to_account_info(),
                authority: self.user.to_account_info()
            };

            let cpi_context =  CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            revoke(cpi_context)?;
            
            //decrement amount staked by 1
            self.user_account.amount_staked -= 1;
        

        Ok(())
    }

}