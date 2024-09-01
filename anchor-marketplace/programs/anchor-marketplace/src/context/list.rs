use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}};

use crate::state::{Marketplace,Listing};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    //Verification on the mint and collection is done in metadata 
    pub mint: Box<InterfaceAccount<'info,Mint>>,
    pub collection_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = seller
    
    )]
    pub seller_mint: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account (
        init,
        payer = seller,
        space = Listing::INIT_SPACE,
        seeds = [
            marketplace.key().as_ref(),
            seller_mint.key().as_ref()],
        bump,
    )]
    pub listing: Account<'info, Listing>,

    //where the mint is stored to be available for purchase.
    #[account(
        init,
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = listing
    )]
    pub vault: Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        seeds = [b"marketplace".as_ref(),marketplace.name.as_str().as_bytes()],
        bump= marketplace.bump,
    )]
    pub marketplace: Account<'info,Marketplace>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            seller_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(), 
        constraint = metadata.collection.as_ref().unwrap().verified == true,
        bump
    )]
    pub metadata: Box<Account<'info, MetadataAccount>>,

    //edition since its master 1 nft
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            seller_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program= metadata_program.key(),
        bump
    )]
    pub edition: Box<Account<'info,MasterEditionAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info,TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info,Metadata>,
}

impl<'info> List<'info> {

    pub fn create_listing(&mut self, price:u64, bumps: &ListBumps) -> Result<()> {

        self.listing.set_inner(Listing {
            seller: self.seller.to_account_info().key(),
            mint: self.mint.to_account_info().key(),
            price,
            bump: bumps.listing
        });

        let cpi_program = self.token_program.to_account_info(); 
        let cpi_accounts = TransferChecked {
            from: self.seller_mint.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.seller.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_context, 1, self.mint.decimals)?;

        Ok(())
    }

    pub fn list(&mut self) -> Result<()> {

        let cpi_program = self.token_program.to_account_info(); 
        let cpi_accounts = TransferChecked {
            from: self.seller_mint.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.seller.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_context, 1, self.mint.decimals)?;

        Ok(())
    }
}