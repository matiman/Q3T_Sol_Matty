use anchor_lang::prelude::*;

pub mod contexts;
pub mod states;
pub mod errors;

pub use contexts::*;
pub use states::*;
pub use errors::*;

declare_id!("CE4kN17mzWxUu9xPRcMqfm8gxtcNYt4qVNVATGnQ5wSW");

#[program]
pub mod dev_loot {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

//architecteral optimization 

//Contexts

//additional bumps (like USDC reward bump)

//Number of PDAs

//WASM .. 

//Vectors ? 



#[derive(Accounts)]
pub struct Initialize {}
