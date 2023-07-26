use anchor_lang::prelude::*;
use notes::program::Notes;
use notes::cpi::{accounts::WriteNote, write_note};

declare_id!("53zmMMp7wu2XJhCj1M17mNgH1c3j9KwmEffXyTNEgkh7");

#[program]
pub mod note_taker {
    use super::*;

    pub fn take_note(ctx: Context<TakeNote>, note: String) -> Result<()> {
        msg!("Making cpi to notes program...");
        write_note(ctx.accounts.take_note_ctx(), note)?;
        msg!("Done!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TakeNote<'info> {
    #[account(mut)]
    pub note: Signer<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub notes_program: Program<'info, Notes>
}

impl<'info> TakeNote<'info> {
    pub fn take_note_ctx(&self) -> CpiContext<'_, '_, '_, 'info, WriteNote<'info>> {
        let cpi_program = self.notes_program.to_account_info();

        let cpi_account = WriteNote{
            authority: self.authority.to_account_info(),
            note: self.note.to_account_info(),
            system_program: self.system_program.to_account_info(),

        };
        CpiContext::new(cpi_program, cpi_account)
    }
}
