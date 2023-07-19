import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterProgram } from "../target/types/counter_program";
import { Keypair, SystemProgram } from "@solana/web3.js";

// Describe the test suite for the counter-program
describe("counter-program", () => {
  // Set the provider to use the environment
  anchor.setProvider(anchor.AnchorProvider.env());
  // Get the provider from the environment
  const provider = anchor.AnchorProvider.env();
  // Get the CounterProgram from the workspace
  const program = anchor.workspace.CounterProgram as Program<CounterProgram>;

  // Create a keypair for the counter account
  const counter = Keypair.generate();

  // Test case to create a Counter account
  it("Create Counter account!", async () => {
    // Call the create method of the program
    const tx = await program.methods.create()
      .accounts({
        counter: counter.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      })
      .signers([counter])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // Test case to increment the Counter
  it("Increment Counter!", async () => {
    // Call the increment method of the program
    const tx = await program.methods.increment()
      .accounts({
        counter: counter.publicKey,
        authority: provider.wallet.publicKey,
      })
      .signers([counter])
      .rpc();
    console.log("Your transaction signature:", tx);
  });

  // Test case to decrement the Counter
  it("Decrement Counter!", async () => {
    // Call the decrement method of the program
    const tx = await program.methods.decrement()
      .accounts({
        counter: counter.publicKey,
        authority: provider.wallet.publicKey,
      })
      .signers([counter])
      .rpc();
    console.log("Your transaction signature:", tx);
  });
});