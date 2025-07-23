use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

#[error_code]
pub enum ErrorCode {
    #[msg("Question too long, maximum 300 characters")]
    QuestionTooLong,
    #[msg("You cannot vote more than once")]
    CannotVoteTwice,
    #[msg("Max amount of votes reached")]
    VoteThreshold,
    #[msg("Poll deadline time invalid. Must be further than the current time and valid UNIX")]
    InvalidExpiryTime,
    #[msg("The poll you are trying to vote on has expired")]
    PollExpired,
}


declare_id!("4Y5yssNBE2pCiKtWYahKJ97LoXEo9gi7bbGRm635Mpj7");

#[program]
pub mod simple_voting_solana {
    use super::*;

    pub fn create_poll(ctx: Context<CreatePoll>, question: String, poll_index: u64, poll_threshold: u64, expiry_time: i64) -> Result<()> {

        // Get current time (UNIX)
        let current_time = Clock::get()?.unix_timestamp;

        // Check time provided is a time after current time
        require!(expiry_time > current_time, ErrorCode::InvalidExpiryTime);
   
        // Check question provided <= 300
        require!(question.len() <= 300, ErrorCode::QuestionTooLong);
        // Set all poll fields
        ctx.accounts.poll.question = question;
        ctx.accounts.poll.yes_votes = 0;
        ctx.accounts.poll.no_votes = 0;
        ctx.accounts.poll.creator = ctx.accounts.creator.key(); // Poll creator
        ctx.accounts.poll.register = Vec::new(); // Hashset to store all voters (avoid d  ouble voting)
        ctx.accounts.poll.poll_threshold = poll_threshold;
        ctx.accounts.poll.created_time = current_time;
        ctx.accounts.poll.expiry_time = expiry_time;
        

        msg!("Initialized poll with PDA: {}", ctx.accounts.poll.key());

        Ok(())
    }

    pub fn vote_for_poll(ctx: Context<VoteForPoll>, vote_choice: bool)-> Result<()>{
        

        let poll = &mut ctx.accounts.poll;
        let voter = &mut ctx.accounts.voter.key();
        // Get current time (UNIX)
        let current_time = Clock::get()?.unix_timestamp;

        //if poll has expired, return error
        if current_time > poll.expiry_time{
            return err!(ErrorCode::PollExpired)
        }

        // If Pubkey already within register vector, return error code
        // Further looking into this, this is an inefficient way to store
        // the public keys of all those who have voted. Solution pending.
        if poll.register.contains(&voter){
            return err!(ErrorCode::CannotVoteTwice);
        }

        // Threshold set at poll creation. If votes (yes+no) goes over our threshold
        // dont process vote.
        // If set to 0, no limit.
        if poll.poll_threshold != 0 && poll.yes_votes + poll.no_votes >= poll.poll_threshold{
            return err!(ErrorCode::VoteThreshold)
            
        }

        // Push voters Pubkey to register vector
        poll.register.push(*voter);

        if vote_choice{
            poll.yes_votes +=1;
        }else{
            poll.no_votes +=1;
        }
        msg!("Vote executed");

        Ok(())
         
    }
}


#[derive(Accounts)]
#[instruction(question: String, poll_index: u64, poll_threshold: u64, expiry_time: i64)]
pub struct CreatePoll<'info> {
    #[account(
        init,
        // PDA using poll_index as a way to create multiple polls
        seeds = [b"poll", creator.key().as_ref(), &poll_index.to_le_bytes()],
        bump,
        // Account creating the poll 
        payer = creator,
        // Discriminator, creator, yes votes, no votes, poll_index, question (string len + max char)
        space = 8 + 32 + 8 + 8 +  8 + (4 + 300),
        
    )]
    pub poll: Account<'info, Poll>,

    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteForPoll<'info>{

    #[account(mut)]
    pub poll: Account<'info, Poll>,
    pub voter: Signer<'info>, 
}


//A poll account
#[account]
pub struct Poll {
    pub question: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub poll_index: u64,
    pub creator: Pubkey,
    pub register: Vec<Pubkey>, // contains Pubkey of those who have voted
    pub poll_threshold: u64,
    pub created_time: i64,
    pub expiry_time: i64,
}


