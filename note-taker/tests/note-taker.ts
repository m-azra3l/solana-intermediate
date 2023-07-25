import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NoteTaker } from "../target/types/note_taker";
import { Keypair, PublicKey, SystemProgram } from '@solana/web3.js'

describe("note-taker", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NoteTaker as Program<NoteTaker>;
  const provider = anchor.AnchorProvider.env()
  const notesProgram = new PublicKey("<program_id-of-your-notes-program>")

  const message = "Hello Metacrafter!"
  const note = Keypair.generate()

  it("Creating note!", async () => {
    // Add your test here.
    const tx = await program.methods.takeNote(message)
    .accounts({
      note: note.publicKey,
      authority: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
      notesProgram: notesProgram
    })
    .signers([note])
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
