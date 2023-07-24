use anchor_lang::prelude::*;

declare_id!("AaJUVVD4yTR1j6EgyP1ZvVTiTVpk8wXk21p4twaFZkrn");

#[program]
pub mod virtual_wallet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
