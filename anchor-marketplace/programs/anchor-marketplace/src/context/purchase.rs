use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token_2022::close_account, token_interface::{ mint_to, transfer_checked, CloseAccount, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked}};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info>{
    #[account(mut)]
    pub buyer: Signer<'info>,

    //seller account where the SOL is sent to.
    #[account(mut)]
    pub seller: SystemAccount<'info>,  

    //mint of the NFT in vault
    pub seller_mint: InterfaceAccount<'info,Mint>,

    //buyer ata where the NFT is sent to.
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = seller_mint,
        associated_token::authority = buyer
    )]
    pub buyer_ata: InterfaceAccount<'info,TokenAccount>,

    //the vault ata where the NFT is sent from.
    #[account(
        mut,
        associated_token::mint = seller_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = seller,
        seeds = [
            marketplace.key().as_ref(),
            seller_mint.key().as_ref()],
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,

    //needed here because listing uses marketplace.key
    #[account(
        seeds = [
            b"marketplace".as_ref(),
            marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    //where the fees from the sell is sent to
    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump
    )]
    pub treasury: SystemAccount<'info>,

    //a reward to be sent to the seller after sell.
    //this is besides the SOL they get from the sell.
    #[account(
        seeds =["rewards".as_ref(),marketplace.key().as_ref()],
        mint::decimals = 6,
        mint::authority = marketplace,
        bump = marketplace.rewards_bump
    )]
    pub rewards_mint: InterfaceAccount<'info,Mint>,

    //create rewards_ata for seller to recieve rewards mint 
    //when purchase by buyer is successful.
    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = rewards_mint,
        associated_token::authority = seller
    )]
    pub seller_rewards_ata: InterfaceAccount<'info,TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    
}

impl<'info> Purchase<'info>{

    //buyer transfers SOL (minus market place fee) to seller.
    pub fn deposit_sol(&mut self) ->Result<()> {

        let ctx_program: AccountInfo<'_> = self.system_program.to_account_info();

        let ctx_accounts = Transfer{
            from: self.buyer.to_account_info(),
            to: self.seller.to_account_info(),
        };

        //todo market place sales fee is % amt instead of dollar amt
        let fee = (
            self.listing.price * 
            self.marketplace.sales_fee as u64
        )/100;

        let sell_price= self.listing.price - fee;

        let cpi_ctx = CpiContext::new(ctx_program, ctx_accounts);

        transfer(cpi_ctx, sell_price * 1_000_000_000)?;//send lamport amount

        Ok(())
    }

    //send NFT from vault to buyer Ata
    pub fn purchase(&mut self) -> Result<()>{

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info() ,
            mint: self.seller_mint.to_account_info(),
            to: self.buyer_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.seller_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx= CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts, 
            signer_seeds
        );
        
        transfer_checked(cpi_ctx, 1, self.seller_mint.decimals)?;

        Ok(())
    }

    //deduct remaining SOL from seller and send it to treasury as a fee
    //for seller they pay what they see on listing price 
    pub fn send_fee_to_treasury(&mut self) -> Result<()>{

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from: self.buyer.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

         //todo market place sales fee is % amt instead of dollar amt
         let fee = (
            self.listing.price * 
            self.marketplace.sales_fee as u64
        )/100;

        transfer(cpi_ctx, fee * 1_000_000_000)?;//lamport

        Ok(())
    }

    //reward seller with 1 rewards_mint from marketplace
    pub fn reward_seller(&mut self) -> Result<()>{

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo{
            mint: self.rewards_mint.to_account_info(),
            to: self.seller_rewards_ata.to_account_info(),
            authority: self.marketplace.to_account_info(),
        };

        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.marketplace.name.as_str().as_bytes()[..],
            &[self.marketplace.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        //mint 1 rewards mint to seller
        mint_to(cpi_ctx, 1)?;

        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()>{

        let cpi_program= self.token_program.to_account_info();

        let cpi_accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.buyer.to_account_info(),
            authority: self.buyer.to_account_info(),
        };

        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.seller_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }
}