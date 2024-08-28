use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, MintTo,mint_to}};

use crate::state::{stake_config::StakeConfig, user_account::UserAccount};

#[derive(Accounts)]
pub struct Claim<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds =[b"user".as_ref(),user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info,UserAccount>,

    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info,StakeConfig>,

    #[account(
        mut,
        seeds = [b"rewards_mint".as_ref(),config.key().as_ref()],
        bump = config.rewards_bump
        
    )]
    //rewards mint exists before calling claim
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user
    )]
    //User ata to recieve rewards mint for
    pub user_rewards_ata: InterfaceAccount<'info,TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    //to init user_rewards_ata
    pub associated_token_program: Program<'info, AssociatedToken>

}

impl<'info> Claim<'info> {
    
    pub fn claim(&mut self) -> Result<()>{

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),
            to: self.user_rewards_ata.to_account_info(),
            authority: self.config.to_account_info()
        };

        let seeds = &[
            b"config".as_ref(),
            &[self.config.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(
            cpi_context,
            self.user_account.points_earned as u64 * 10_u64.pow(self.rewards_mint.decimals as u32))?;

        Ok(())
    }
}