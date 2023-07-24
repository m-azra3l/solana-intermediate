use anchor_lang::prelude::*;

declare_id!("D4vXUMbHSF3Qj6ChxsZpnNxNXVFEQTy4aPmhuWjnQZJk");

#[program]
pub mod notes {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
