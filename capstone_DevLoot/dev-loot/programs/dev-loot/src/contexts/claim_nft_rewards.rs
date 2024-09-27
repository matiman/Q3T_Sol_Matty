use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, MintTo,mint_to}};

use crate::{states::stake_config::StakeConfig, StudentPrgress};

#[derive(Accounts)]
pub struct Claim<'info>{
    #[account(mut)]
    pub student: Signer<'info>,

    #[account(
        mut,
        seeds =[b"student_progress".as_ref(),student.key().as_ref()],
        bump = student_progress.bump
    )]
    pub student_progress: Account<'info,StudentPrgress>,

    #[account(
        seeds = [b"stake_config".as_ref()],
        bump = stake_config.bump
    )]
    pub stake_config: Account<'info,StakeConfig>,

    #[account(
        mut,
        seeds = [b"stake_rewards_mint".as_ref(),stake_config.key().as_ref()],
        bump = stake_config.rewards_bump
        
    )]
    //rewards mint exists before calling claim
    pub stake_rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = student,
        associated_token::mint = stake_rewards_mint,
        associated_token::authority = student
    )]
    //User ata to recieve rewards mint for
    pub student_stake_rewards_ata: InterfaceAccount<'info,TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    //to init user_rewards_ata
    pub associated_token_program: Program<'info, AssociatedToken>

}

impl<'info> Claim<'info> {
    
    pub fn claim(&mut self) -> Result<()>{

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.stake_rewards_mint.to_account_info(),
            to: self.student_stake_rewards_ata.to_account_info(),
            authority: self.stake_config.to_account_info()
        };

        let seeds = &[
            b"config".as_ref(),
            &[self.stake_config.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        let staking_points_earned =self.student_progress.staking_points_earned as u64;

        mint_to(
            cpi_context,
            staking_points_earned.checked_mul(
                10_u64.checked_pow(self.stake_rewards_mint.decimals as u32).unwrap()).unwrap())?;

        Ok(())
    }
}