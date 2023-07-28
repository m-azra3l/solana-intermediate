import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AirdropProgram } from "../target/types/airdrop_program";
import { Keypair, SystemProgram, PublicKey } from '@solana/web3.js'
import { TOKEN_PROGRAM_ID } from '@solana/spl-token'

describe("airdrop-program", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.AirdropProgram as Program<AirdropProgram>;
  const provider = anchor.AnchorProvider.env()

  const [tokenMint, mintBump] = await PublicKey.findProgramAddress(
    [Buffer.from("token-mint")],
    program.programId
  )

  const [mintAuth, authBump] = await PublicKey.findProgramAddress(
    [Buffer.from("mint-authority")],
    program.programId
  )
  console.log("Token mint pda: ", tokenMint.toBase58())
  console.log("Mint authority pda: ", mintAuth.toBase58())


  it("Create token mint", async () => {
    // Add your test here.
    const tx = await program.methods.initializeMint(10)
    .accounts({
      tokenMint: tokenMint,
      mintAuthority: mintAuth,
      payer: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Airdrop tokens", async () => {
    let userTokenAccount = await Keypair.generate()
    const tx = await program.methods.airdrop(new anchor.BN(12))
    .accounts({
      tokenMint: tokenMint,
      mintAuthority: mintAuth,
      user: provider.wallet.publicKey,
      userTokenAccount: userTokenAccount.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([userTokenAccount])
    .rpc()

    console.log("Airdrop tx: ", tx)
  })

  it("Airdrop more tokens", async () => {
    let userTokenAccount = await Keypair.generate()
    const tx = await program.methods.airdrop(new anchor.BN(25))
    .accounts({
      tokenMint: tokenMint,
      mintAuthority: mintAuth,
      user: provider.wallet.publicKey,
      userTokenAccount: userTokenAccount.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([userTokenAccount])
    .rpc()

    console.log("Airdrop tx: ", tx)
  })
});