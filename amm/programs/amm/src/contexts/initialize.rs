use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint,
        MintTo,
        TokenAccount, 
        TokenInterface,
        TransferChecked,
        transfer_checked,
        mint_to
    }};

use crate::state::config::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    mint_x: Box<InterfaceAccount<'info, Mint>>,
    mint_y: Box<InterfaceAccount<'info,Mint>>,

    #[account(
        init,
        payer = maker,
        space = 8 + Config::INIT_SPACE,
        seeds = [
            b"amm",mint_x.key().as_ref(),
            mint_y.key().as_ref(),
            seed.to_le_bytes().as_ref()],
        bump
    )]
    pub config: Box<Account<'info,Config >>,

    #[account(
        init_if_needed,
        payer = maker,
        mint::authority = config,
        mint::decimals = 6,
        mint::token_program = token_program, // this can change depending on if we need token 2022
        seeds = [b"mint", config.key().as_ref()],
        bump
    )]
    pub mint_lp: Box<InterfaceAccount<'info,Mint>>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_x_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    pub vault_y_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_x,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_x_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_y,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_y: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_lp,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_lp: Box<InterfaceAccount<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> Initialize<'info> {

    pub fn save_config(&mut self, seed:u64,fee: u16, bump: u8, lp_bump: u8) -> Result<()>{
        self.config.set_inner(
            Config {
                seed,
                fee,
                mint_x: self.mint_x.key(),
                mint_y: self.mint_y.key(),
                lp_bump,
                bump, 
            }
        );

        Ok(())

    }
    pub fn deposit(&self, amount: u64,is_x: bool) -> Result<()>{
        let (from, to, mint, decimals) = match is_x {
            true => {(
                self.maker_x_ata.to_account_info(),
                self.vault_x_ata.to_account_info(),
                self.mint_x.to_account_info(), 
                self.mint_x.decimals )},
            false => {(
                self.maker_ata_y.to_account_info(),
                self.vault_y_ata.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals )},
        };

        let accounts = TransferChecked {
            from,
            to,
            mint: mint.to_account_info(),
            authority: self.maker.to_account_info()
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, accounts);

        transfer_checked(cpi_ctx, amount, decimals)?;

        Ok(())

    }

    pub fn mint_lp_token(&mut self, amount_x:u64, amount_y:u64   ) -> Result<()> {

        let accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.maker_ata_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };
        
        //TODO Unless I move out the variables like below, signer seeds had some issues using as_ref()
        let seed = self.config.seed.to_le_bytes();
        let bump = [self.config.bump];
        let mint_x = self.mint_x.to_account_info().key();
        let mint_y = self.mint_y.to_account_info().key();

        let signer_seeds = [&[
            b"amm",
            mint_x.as_ref(),
            mint_y.as_ref(),
            seed.as_ref(),
            &bump
        ][..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, accounts, &signer_seeds);

        let lp_amount = amount_x.checked_mul(amount_y).
                                            ok_or(ProgramError::ArithmeticOverflow)?;

        mint_to(cpi_ctx, lp_amount)?;
        Ok(())
    }
    
}