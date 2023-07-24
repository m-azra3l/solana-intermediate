use anchor_lang::prelude::*;

declare_id!("D4vXUMbHSF3Qj6ChxsZpnNxNXVFEQTy4aPmhuWjnQZJk");

#[program]
pub mod notes {
    use super::*;

    pub fn write_note(ctx: Context<WriteNote>) -> Result<()> {
        msg!("Creating a note inside notes program...");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WriteNote {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32
    )]
    pub note: Account<'info, Note>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Note {
    pub authority: Pubkey,
    pub note: String,
}
