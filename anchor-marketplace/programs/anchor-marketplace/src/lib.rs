use anchor_lang::prelude::*;

pub mod state;
pub mod context;
pub mod errors;

pub use state::*;
pub use context::*;

declare_id!("Ab2u3sAxHSdZ3w3fnj9NKT3hpa3PeQfk516EnAuoKmAV");

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn list(ctx: Context<List>, price:u64 ) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.list_nft()?;
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.deposit_sol()?;
        ctx.accounts.purchase()?;
        ctx.accounts.send_fee_to_treasury()?;
        ctx.accounts.reward_seller()?;
        
        Ok(())
    }

    pub fn delist_and_close(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.transfer_nft_and_close()?;
        Ok(())
    }
}
