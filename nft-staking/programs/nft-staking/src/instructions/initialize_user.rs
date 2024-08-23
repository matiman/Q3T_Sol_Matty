use anchor_lang::prelude::*;

use crate::state::user_account::UserAccount;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = UserAccount::INIT_SPACE,
        seeds = [b"user",user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info,UserAccount>,

    pub system_program: Program<'info, System>

}

impl<'info> InitializeUser<'info> {
    pub fn initialize(&mut self, bumps: &InitializeUserBumps) -> Result<()>{
        
        self.user_account.set_inner(UserAccount {
            points_earned: 0,
            amount_staked: 0,
            bump: bumps.user_account

        });

        Ok(())
        
    }
}