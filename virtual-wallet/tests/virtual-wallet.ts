import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VirtualWallet } from "../target/types/virtual_wallet";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import assert from "assert";

describe("virtual-wallet", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.VirtualWallet as Program<VirtualWallet>;
  const provider = anchor.AnchorProvider.env();

  it("Initialize Virtual Wallet!", async () => {
    // Add your test here.

    // derive cash pda
    const [cash, cashBump]  = await PublicKey.findProgramAddress(
      [provider.wallet.publicKey.toBuffer(), Buffer.from("cash")], 
      program.programId,
    );

    // derive credit card pda
    const [creditCard, ccBump]  = await PublicKey.findProgramAddress(
      [provider.wallet.publicKey.toBuffer(), Buffer.from("credit-card")], 
      program.programId,
    );

    // derive dedit card pda
    const [deditCard, dbBump]  = await PublicKey.findProgramAddress(
      [provider.wallet.publicKey.toBuffer(), Buffer.from("dedit-card")], 
      program.programId,
    );

    const tx = await program.methods.initializeWallet()
    .accounts({
      user: provider.wallet.publicKey,
      cash: cash,
      creditCard: creditCard,
      deditCard: deditCard,
      systemProgram: SystemProgram.programId
    })
    .rpc();
    console.log("Virtual Wallet Initialized!");
    console.log("Your transaction signature", tx);
  });
});
