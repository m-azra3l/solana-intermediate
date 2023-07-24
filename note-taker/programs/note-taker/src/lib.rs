use anchor_lang::prelude::*;

declare_id!("53zmMMp7wu2XJhCj1M17mNgH1c3j9KwmEffXyTNEgkh7");

#[program]
pub mod note_taker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
