use anchor_lang::prelude::*;

pub mod state;
pub mod contexts;
pub use contexts::*;

declare_id!("GyakaPBCVSDfzTMtGL5mbfjuTiJY3kWAPoG9Gox4j16B");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee:u16, amount_x: u64, amount_y: u64) -> Result<()> {
        //save_config
        ctx.accounts.save_config(seed, fee, ctx.bumps.config, ctx.bumps.mint_lp)?;
                
        //intial deposit to bootstrap liquidity
        ctx.accounts.deposit(amount_x, true)?;
        // ctx.accounts.deposit(amount_y, false)?;
        
        Ok(())
    }

    // //Deposit liquidity to mint lp_token
    // pub fn deposit(ctx: Context<Deposit>, lp_amount:u64, max_x:u64, max_y:u64) -> Result<()> {
    //     //deposit tokens(2x)
    //     //mint lp_token
    //     Ok(())

    // }

    // //burn lp token to withdraw liquidity
    // pub fn withdraw(ctx: Context<Withdraw>, lp_amount:u64, min_x:u64, min_y:u64) -> Result<()> {
    //     //burn lp token
    //     //withdraw liquidity

    //     Ok(())

    // }

    // //is_x is to know which token is coming so we know which token to swap to
    // //TODO a max time limit
    // pub fn swap(ctx: Context<Swap>, amount_to_swap: u64, min_recieve:u64, is_x:bool) -> Result<()>{
    //     //deposit token
    //     //withdraw token
    //     Ok(())

    // }
}

