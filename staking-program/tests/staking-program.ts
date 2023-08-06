import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Token, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import assert from "assert";

import { StakingProgram } from '../target/types/staking_program';

describe('staking-program', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.StakingProgram as Program<StakingProgram>;
  const mintAuthority = Keypair.generate();
  let mint: PublicKey;
  let pool: PublicKey;

  before(async () => {
    // Create a new mint.
    mint = await Token.createMint(
      program.provider,
      mintAuthority.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID,
      SystemProgram.programId
    );

    // Create a new pool.
    const newPoolTx = new anchor.web3.Transaction();
    const newPoolIx = await program.account.pool.createInstruction(pool, 1000);
    newPoolTx.add(newPoolIx);
    await program.provider.send(newPoolTx);
  });

  it('stakes and unstakes tokens', async () => {
    const user = Keypair.generate();
    const amount = new anchor.BN(100);

    // Create a new token account for the user.
    const userTokenAccount = await Token.createAssociatedTokenAccount(
      TOKEN_PROGRAM_ID,
      program.programId,
      mint,
      user.publicKey,
    );

    // Mint some tokens to the user's token account.
    const mintTokenIx = Token.createMintToInstruction(
      TOKEN_PROGRAM_ID,
      mint,
      userTokenAccount,
      mintAuthority.publicKey,
      [],
      amount.toNumber(),
    );
    await program.provider.send(new anchor.web3.Transaction().add(mintTokenIx));

    // Stake the tokens.
    await program.rpc.stake(amount, {
      accounts: {
        poolSpt: pool,
        from: userTokenAccount,
        user: user.publicKey,
        pool: pool,
        staker: userTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    });

    // Check that the user's staked amount is correct.
    const userStakeAccount = await program.account.stakeAccount.fetch(
      user.publicKey,
      'state_account'
    );
    assert.equal(userStakeAccount.amount.toNumber(), amount.toNumber());

    // Unstake the tokens.
    await program.rpc.unstake(amount, {
      accounts: {
        poolSpt: pool,
        from: userTokenAccount,
        user: user.publicKey,
        pool: pool,
        staker: userTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    });

    // Check that the user's staked amount is now 0.
    const updatedUserStakeAccount = await program.account.stakeAccount.fetch(
      user.publicKey,
      'state_account'
    );
    assert.equal(updatedUserStakeAccount.amount.toNumber(), 0);
  });
});