use anchor_lang::prelude::*;

declare_id!("9vVwobbUQKqweePif76gvLBgg3uR12P8Ssae2x73kkFp");

#[program]
pub mod counter_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
