use anchor_lang::prelude::*;

declare_id!("4wNomB8Ncca1ddQTSp9V9L4tV8bkS4nkHHWyMBZSTFNQ");

#[program]
pub mod staking_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
