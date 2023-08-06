use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("4wNomB8Ncca1ddQTSp9V9L4tV8bkS4nkHHWyMBZSTFNQ");

#[program]
pub mod staking_program {
    use super::*;

    pub fn new_pool(ctx: Context<NewPool>, bump: u8) -> ProgramResult {
        let pool = &mut ctx.accounts.pool;
        pool.bump = bump;
        pool.mint = *ctx.accounts.mint.to_account_info().key;
        pool.owner = *ctx.accounts.owner.key;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> ProgramResult {
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.staker.to_account_info().clone(),
            to: ctx.accounts.from.to_account_info().clone(),
            authority: ctx.accounts.pool_spt.to_account_info().clone(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info().clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct NewPool<'info> {
    #[account(init, payer = owner, seeds = ['pool.as_bytes(), &[bump]])]
    pool: Account<'info, Pool>,
    owner: Signer<'info>,
    system_program: Program<'info, System>,
    #[account(constraint = mint.mint_authority.get()? == pool.to_account_info().key)]
    mint: CpiAccount<'info, Mint>,
}

#[account]
pub struct Pool {
    bump: u8,
    mint: Pubkey,
    owner: Pubkey,
    stake_distribution: u64,
    total_staker_balance: u64,
    staker_count: u64,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut, has_one = mint)]
    pool_spt: Account<'info, TokenAccount>,

    #[account(mut, has_one = mint)]
    from: Account<'info, TokenAccount>,

    // Has to be signer to send tokens from.
    #[account(signer)]
    user: AccountInfo<'info>,

    #[account(seeds = [pool_spt.key.as_ref()])]
    pool: Account<'info, Pool>,

    #[account(
        init_if_needed,
        associated_token::authority,
        with = pool,
    )]
    staker: Account<'info, TokenAccount>,

    // Programs needed for CPI.
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut, has_one = mint)]
    pool_spt: Account<'info, TokenAccount>,

    #[account(mut, has_one = mint)]
    from: Account<'info, TokenAccount>,

    // Has to be signer to send tokens from.
    #[account(signer)]
    user: AccountInfo<'info>,

    #[account(seeds = [pool_spt.key.as_ref()])]
    pool: Account<'info, Pool>,

    #[account(
        init_if_needed,
        associated_token::authority,
        with = pool,
    )]
    staker: Account<'info, TokenAccount>,

    // Programs needed for CPI.
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}