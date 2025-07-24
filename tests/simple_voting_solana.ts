import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleVotingSolana } from "../target/types/simple_voting_solana";
import { BN } from "@coral-xyz/anchor";
import { expect } from "chai"; // Better assertion testing
import { Keypair, PublicKey } from "@solana/web3.js"; // Create test users using keypair.generate()

// Generate Poll Id using random for test case
// Using Big Number for u64 compatibility
const generatePollId = () => new BN(Math.floor(Math.random() * 10000));

// Time in UNIX when the poll expires
const pollExpireTime = new BN(2753299200);

// Generate the PDA (Program Derived Address) for poll
// Seeds: "poll" + creator's pubkey + poll_index
const getPollPDA = (creator: PublicKey, pollId: BN): [PublicKey, number] => {
    const program = anchor.workspace.SimpleVotingSolana;
    return anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("poll"),
            creator.toBuffer(),
            pollId.toBuffer("le", 8),
        ],
        program.programId
    );
};

// Simple Voting Solana test block
describe("Simple Voting Solana Program", () => {
    // Setup connection to my Solana local validator
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.SimpleVotingSolana;
    // Test poll creation
    describe("Poll Creation", () => {
        it("Should create poll successfully", async () => {
            const pollID = generatePollId();
            const question = "Do you like cake?";
            // Vote threshold, stops voting once met.
            const voteThreshold = new BN(3);

            //create poll
            const [pollPDA] = getPollPDA(provider.wallet.publicKey, pollID);

            // Call create_poll instruction
            // Initialize new poll account at PDA address
            await program.methods
                .createPoll(question, pollID, voteThreshold, pollExpireTime)
                .accounts({
                    poll: pollPDA,
                    creator: provider.wallet.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .rpc(); // rpc used to send the transaction :)

            const pollAccount = await program.account.poll.fetch(pollPDA);
            expect(pollAccount.creator.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
            expect(pollAccount.question).to.equal(question); // Check question
            expect(pollAccount.yesVotes.toNumber()).to.equal(0); // Yes votes should start at 0
            expect(pollAccount.noVotes.toNumber()).to.equal(0);  // No votes should start at 0
        });
    });

    // Test voting instruction
    describe("Poll Voting", () => {
        const pollID = generatePollId();
        const [pollPDA] = getPollPDA(provider.wallet.publicKey, pollID);

        // runs before each test, creates a new poll every time.
        beforeEach(async () => {
            // Create a fresh poll before each voting test
            const question = "Do you like cake?";
            const voteThreshold = new BN(3);
            //create poll
            await program.methods
                .createPoll(question, pollID, voteThreshold, pollExpireTime)
                .accounts({
                    poll: pollPDA,
                    creator: provider.wallet.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .rpc();
        });

        it("should allow voting 'yes' on a poll", async () => {
            await program.methods.voteForPoll(true) // Vote YES
                .accounts({
                    poll: pollPDA,
                    voter: provider.wallet.publicKey,
                })
                .rpc();

            const pollAccount = await program.account.poll.fetch(pollPDA);
            expect(pollAccount.yesVotes.toNumber()).to.equal(1); //since voted yes (true): yesVotes += 1
            expect(pollAccount.noVotes.toNumber()).to.equal(0);
        });
    });
});


