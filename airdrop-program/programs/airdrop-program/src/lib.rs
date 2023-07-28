use anchor_lang::prelude::*;

declare_id!("Hc6u2NbmsLnBXMr5DkWYFxUab2FUzfGporZLpYDpkVp1");

#[program]
pub mod airdrop_program {
    use super::*;

    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        Ok(())
    }

    pub fn airdrop(ctx: Context<Airdrop>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint {}

#[derive(Accounts)]
pub struct Airdrop {}