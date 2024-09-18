use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("3ZGEdc6AEUqJSokEGwfXL4kqXLWdiHYTizd5tvExQhoj");

#[program]
pub mod anchor_vault_q3_2024 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;//question mark is to pass error
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount:u64) -> Result<()>{
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount:u64)-> Result<()>{
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close_vault(ctx: Context<Close>)-> Result<()>{
        ctx.accounts.close_vault()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer =user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info,VaultState>,

    #[account(
        seeds = [b"vault",vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info,System>

}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info,VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>

}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        //vault_state will be closed and we don't transfer lamport out of it
        //like we do with vault in withdraw method
        //close anchor attribute takes care of, transfering all lamports,
        //zeroing out account data
        close = user,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info,VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>

}

impl <'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()>{

        self.vault_state.state_bump= bumps.vault_state;
        self.vault_state.vault_bump= bumps.vault;

        Ok(())
    }
    
}

impl <'info> Payment<'info> {
    //deposit lamport from user to vault
    pub fn deposit(&mut self, amount: u64) -> Result<()>{
        let cpi_program: AccountInfo = self.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

impl <'info> Payment<'info> {
    //withdraw lamport from vault to user if available
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {

        //Here the signer seeds is picking up vault, vault state key, and vault bump
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program: AccountInfo = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };

        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program, cpi_accounts, signer_seeds);

        let balance= self.vault.lamports();

        //TODO Throw an error if amount is > balance
        if amount<balance {
            transfer(cpi_ctx,amount)?;
        }

        Ok(())

    }
}

impl <'info> Close<'info> {
    //close vault by transfer all lamports back to user.
    pub fn close_vault(&mut self) -> Result<()> {

        //Here the signer seeds is picking up vault, vault state key, and vault bump
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program: AccountInfo = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };

        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program, cpi_accounts, signer_seeds);

        let balance= self.vault.lamports();

        //transfer all balance back to user.
        transfer(cpi_ctx,balance)?;

        Ok(())

    }
}

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8
}
impl Space for VaultState {
    const INIT_SPACE:usize = 8 + 1 + 1;
}