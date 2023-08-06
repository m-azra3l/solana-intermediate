import * as anchor from '@project-serum/anchor';
import { Token, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { SystemProgram } from '@solana/web3.js';
import assert from 'assert';

import { StakingProgram } from '../target/types/staking_program';

describe('staking-program', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StakingProgram as anchor.Program<StakingProgram>;
  const mintAuthority = anchor.web3.Keypair.generate();
  let mint: anchor.web3.PublicKey;
  let pool: anchor.web3.PublicKey;

  before(async () => {
    // Create a new mint.
    mint = await Token.createMint(
      program.provider,
      mintAuthority.publicKey,
      null,
      null,
      TOKEN_PROGRAM_ID,
      SystemProgram.programId
    );

    // Create a new pool.
    pool = anchor.web3.Keypair.generate().publicKey;
    await program.rpc.new_pool(new anchor.BN(0), {
      accounts: {
        pool,
        owner: program.provider.wallet.publicKey,
        mint,
        systemProgram: SystemProgram.programId,
      },
      signers: [mintAuthority],
    });
  });

  it('stakes and unstakes tokens', async () => {
    const user = anchor.web3.Keypair.generate();
    const amount = new anchor.BN(100);

    // Create a new token account for the user.
    const userTokenAccount = await Token.getAssociatedTokenAddress(
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
        stakingPool: pool,
        from: userTokenAccount,
        user: program.provider.wallet.publicKey,
        staker: userTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [user],
    });

    // Check that the user's staked amount is correct.
    const userStakeAccount = await program.account.staker.fetch(
      user.publicKey,
      userTokenAccount,
    );
    assert.equal(userStakeAccount.staked_balance.toNumber(), amount.toNumber());

    // Unstake the tokens.
    await program.rpc.unstake(amount, {
      accounts: {
        stakingPool: pool,
        from: userTokenAccount,
        user: program.provider.wallet.publicKey,
        staker: userTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [user],
    });

    // Check that the user's staked amount is now 0.
    const updatedUserStakeAccount = await program.account.staker.fetch(
      user.publicKey,
      userTokenAccount,
    );
    assert.equal(updatedUserStakeAccount.staked_balance.toNumber(), 0);
  });
});