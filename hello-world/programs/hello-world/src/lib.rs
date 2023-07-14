use anchor_lang::prelude::*;

declare_id!("81BZshqgucT57iAw4kgPZiGpApZS4LbBzTXT2UMEAzod");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello Metacrafter!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
