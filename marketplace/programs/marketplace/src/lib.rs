use anchor_lang::prelude::*;

declare_id!("9F6KWdJkBJvos2dR8TE8CVrwfMLM4D6LnwBGT7aEcR4T");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
