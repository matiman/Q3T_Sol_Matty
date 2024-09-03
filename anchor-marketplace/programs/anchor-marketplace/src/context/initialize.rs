use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        space = Marketplace::INIT_SPACE,
        seeds = [b"marketplace".as_ref(), name.as_str().as_bytes()],
        bump
    )]
    pub marketplace: Account<'info,Marketplace>,

    //to collect fees in SOL when purchase happens
    //should be intialized in test ? 
    #[account(
        seeds = [b"treasury".as_ref(), marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        seeds =[b"rewards".as_ref(), marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,


    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info,TokenInterface>
    
}

impl<'info> Initialize<'info> {

    pub fn initialize(&mut self, name: String, sales_fee: u16, bumps: &InitializeBumps) -> Result<()>{

        self.marketplace.set_inner(Marketplace{
            admin: self.admin.key(),
            name,
            sales_fee,
            bump: bumps.marketplace,
            rewards_bump: bumps.rewards_mint,
            treasury_bump: bumps.treasury
        });

        Ok(())

    }
    
}