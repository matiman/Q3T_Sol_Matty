use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TransferChecked,TokenInterface, transfer_checked};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info>{
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = seller_mint,
        associated_token::authority = seller
    )]
    pub seller_ata: InterfaceAccount<'info,TokenAccount>,

    pub seller_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = seller_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = seller,
        seeds = [marketplace.key().as_ref(),seller_mint.key().as_ref()],
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        seeds = [b"marketplace",marketplace.name.as_str().as_bytes() ],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Delist<'info> {

    pub fn transfer_nft_and_close(&mut self)-> Result<()>{

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info(),
            mint: self.seller_mint.to_account_info(),
            to: self.seller_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.seller_mint.key().to_bytes()[..],
            &[self.listing.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.vault.amount, self.seller_mint.decimals)?;

        Ok(())
    
    }
}