use anchor_lang::prelude::*;

declare_id!("8ftYsC7GGH6sh1MQw81TQBc6W4TR3gsLp7it4vFNDxUu");

#[program]
pub mod dev_loot {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
