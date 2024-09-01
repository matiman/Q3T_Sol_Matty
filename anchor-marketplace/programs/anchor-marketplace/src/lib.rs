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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
