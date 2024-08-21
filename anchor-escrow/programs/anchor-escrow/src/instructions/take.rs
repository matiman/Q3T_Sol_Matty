use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, 
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked,CloseAccount,
        transfer_checked, close_account
    }};

use crate::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    // System Account b/c we transfer rent lamport to maker when we close the escrow
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,


    #[account(
        //init if needed because taker's mint a ata might not have been intialized
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker

    )]
    pub taker_ata_a: Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker

    )]
    pub taker_ata_b: Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        //init if needed because maker's mint b ata might not have been intialized
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker 
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        mut,
        //we close escrow once take function is complete and maker gets back rent
        close = maker,
        //has_one is used to check the "mints and make" in the escrow to match what we have here
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    #[account(
        //mut b/c vault will transfer the token to taker
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub vault_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    //For creation/intialization of accounts
    pub system_program: Program<'info, System>,
    //since we are using intiating atas
    pub associated_token_program: Program<'info, AssociatedToken>,
    //to transfer spl tokens (for mint in this case)
    pub token_program: Interface<'info, TokenInterface>

}

impl <'info> Take<'info> {

    pub fn deposit_to_maker(&mut self) -> Result<()> {

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            authority: self.taker.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, self.escrow.amount_to_recieve, self.mint_b.decimals)?;

        Ok(())
    }

    pub fn take(&mut self) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();
        
        let cpi_accounts = TransferChecked {
            from: self.vault_ata_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let signer_seeds: [&[&[u8]];1] = [
                    &[
                        b"escrow", 
                        self.maker.to_account_info().key.as_ref(), 
                        &self.escrow.seed.to_le_bytes()[..],
                        &[self.escrow.bump]
                    ]
                ];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        transfer_checked(cpi_ctx, self.vault_ata_a.amount , self.mint_a.decimals)?;
        Ok(())
    }

    pub fn close_account(&mut self,) -> Result<()>{
        //TODO close vault

        let cpi_program = self.token_program.to_account_info();
        
        let cpi_accounts = CloseAccount {
            account: self.vault_ata_a.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let signer_seeds: [&[&[u8]];1] = [
                    &[
                        b"escrow", 
                        self.maker.to_account_info().key.as_ref(), 
                        &self.escrow.seed.to_le_bytes()[..],
                        &[self.escrow.bump]
                    ]
                ];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        
        close_account(cpi_ctx)?;

        Ok(())
    }
    
}
