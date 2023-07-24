use anchor_lang::prelude::*;

declare_id!("AaJUVVD4yTR1j6EgyP1ZvVTiTVpk8wXk21p4twaFZkrn");

#[program]
pub mod virtual_wallet {
    use super::*;

    pub fn initialize_wallet(ctx: Context<InitializeWallet>) -> Result<()> {

        ctx.accounts.cash.amount = 0;

        ctx.accounts.credit_card.credit_used = 0;
        ctx.accounts.credit_card.credit_limit = 5000;

        ctx.accounts.dedit_card.checking = 0;
        ctx.accounts.dedit_card.savings = 0;

        msg!("Initialized virtual wallet");
        msg!("Credit card limit: {}", ctx.accounts.credit_card.credit_limit);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeWallet<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, 
        seeds = [user.key().as_ref(),b"cash"],
        bump,
        payer = user,
        space = 8 + 8
    )]
    pub cash: Account<'info, Cash>,

    #[account(
        init, 
        seeds = [user.key().as_ref(),b"credit-card"],
        bump,
        payer = user,
        space = 8 + 16
    )]
    pub credit_card: Account<'info, CreditCard>,
    #[account(
        init, 
        seeds = [user.key().as_ref(),b"dedit-card"],
        bump,
        payer = user,
        space = 8 + 16
    )]
    pub dedit_card: Account<'info, DeditCard>,

    pub system_program: Program<'info, System>
}

#[account]
pub struct Cash {
    pub amount: u64
}

#[account]
pub struct CreditCard {
    pub credit_used: u64,
    pub credit_limit: u64
}

#[account]
pub struct DeditCard {
    pub checking: u64,
    pub savings: u64
}
