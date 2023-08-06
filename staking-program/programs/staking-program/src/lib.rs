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

        // Initialize the staking pool token account with PDA as the authority.
        let seeds = &[b"staking_pool", &pool.mint.as_ref(), &[pool.bump]];
        let (staking_pool_key, bump) = Pubkey::find_program_address(seeds, ctx.program_id);

        let create_account_ix = token::create_account(
            &ctx.accounts.token_program.key,
            &ctx.accounts.owner.key,
            &staking_pool_key,
            &ctx.accounts.pool_spt.key,
            &bump,
            token::Token::get_minimum_balance_for_rent_exemption(&ctx.accounts.token_program)?,
            token::Mint::LEN as u64,
            &ctx.program_id,
        )?;

        invoke_signed(
            &create_account_ix,
            &[ctx.accounts.token_program.clone(), ctx.accounts.owner.clone()],
            &[&seeds],
        )?;

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> ProgramResult {
        // Transfer tokens from the user's token account to the program's token account.
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info().clone(),
            to: ctx.accounts.staking_pool.to_account_info().clone(),
            authority: ctx.accounts.user.to_account_info().clone(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info().clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        // Update the user's staked balance in their additional account.
        ctx.accounts.staker.staked_balance += amount;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> ProgramResult {
        // Ensure the user has staked enough tokens to unstake.
        if amount > ctx.accounts.staker.staked_balance {
            return Err(ErrorCode::InsufficientStake.into());
        }

        // Transfer tokens from the program's token account to the user's token account.
        let cpi_accounts = Transfer {
            from: ctx.accounts.staking_pool.to_account_info().clone(),
            to: ctx.accounts.from.to_account_info().clone(),
            authority: ctx.accounts.pool_spt.to_account_info().clone(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info().clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        // Update the user's staked balance in their additional account.
        ctx.accounts.staker.staked_balance -= amount;
        Ok(())
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
    pool_spt: Account<'info, TokenAccount>,
    #[account(signer)]
    rent: Sysvar<'info, Rent>,
}

#[account]
pub struct Pool {
    bump: u8,
    mint: Pubkey,
    owner: Pubkey,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut, has_one = mint)]
    staking_pool: Account<'info, TokenAccount>,

    #[account(mut, has_one = mint)]
    from: Account<'info, TokenAccount>,

    // Has to be signer to send tokens from.
    #[account(signer)]
    user: AccountInfo<'info>,

    #[account(
        init_if_needed,
        associated_token::authority,
        with = staking_pool,
    )]
    staker: Account<'info, Staker>,

    // Programs needed for CPI.
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Staker {
    staked_balance: u64,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut, has_one = mint)]
    staking_pool: Account<'info, TokenAccount>,

    #[account(mut, has_one = mint)]
    from: Account<'info, TokenAccount>,

    // Has to be signer to send tokens from.
    #[account(signer)]
    user: AccountInfo<'info>,

    #[account(
        init_if_needed,
        associated_token::authority,
        with = staking_pool,
    )]
    staker: Account<'info, Staker>,

    // Programs needed for CPI.
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}

#[error]
pub enum ErrorCode {
    #[msg("Insufficient stake amount")]
    InsufficientStake,
}