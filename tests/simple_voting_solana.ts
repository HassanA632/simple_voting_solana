import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleVotingSolana } from "../target/types/simple_voting_solana";
import { BN } from "@coral-xyz/anchor";

// Test data (question for poll and poll index)
const question = "Do you like cake?";
// Using Big Number for u64 compatibility
const pollID = new BN(6)

describe("poll program test", () => {
  it("Creates poll successfully", async () => {
    // Set up Anchor provider and program connection
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.SimpleVotingSolana;

    // Get wallet public key
    const creatorPublicKey = provider.wallet.publicKey;





    // Generat e the PDA (Program Derived Address) for poll
    // Seeds: "poll" + creator's pubkey + poll_index
    const [pollPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("poll"),           // Static seed
        creatorPublicKey.toBuffer(),   // Creator's public key as seed
        pollID.toBuffer("le", 8),      // Poll index as 8-byte little-endian
      ],
      program.programId
    );


    console.log("PDA WE ARE SENDING: ", pollPDA.toBase58(), "bump:", bump);
    console.log("NUMBER: ", pollID)

    // Call create_poll instruction
    // Initialize new poll account at PDA address
    await program.methods
      .createPoll(question, pollID)
      .accounts({
        poll: pollPDA,
        creator: creatorPublicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();


    const pollAccount = await program.account.poll.fetch(pollPDA);

    // Display poll data to confirm
    console.log("> Poll creator:", pollAccount.creator.toBase58());
    console.log("> question: ", pollAccount.question);
    console.log("> Yes:", pollAccount.yesVotes);
    console.log("> No:", pollAccount.yesVotes);

  });
});