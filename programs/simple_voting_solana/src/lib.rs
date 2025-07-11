use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Question too long, maximum 300 characters")]
    QuestionTooLong,
}

declare_id!("4Y5yssNBE2pCiKtWYahKJ97LoXEo9gi7bbGRm635Mpj7");

#[program]
pub mod simple_voting_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_poll(ctx: Context<CreatePoll>, question: String, poll_index: u64) -> Result<()> {
        //Check question provided <= 300
        require!(question.len() <= 300, ErrorCode::QuestionTooLong);
        //Set all poll fields
        ctx.accounts.poll.question = question;
        ctx.accounts.poll.poll_index = poll_index;
        ctx.accounts.poll.yes_votes = 0;
        ctx.accounts.poll.no_votes = 0;
        ctx.accounts.poll.creator = ctx.accounts.creator.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account(init,
        //account creating the poll
        payer = creator,
        //discriminator, creator, yes votes, no votes, poll_index, question (string len + max char)
        space = 8 + 32 + 8 + 8 +  8 (4 + 300),
        //PDA using poll_index as a way to create multiple polls
        seeds = [creator.key().as_ref(), &poll_index.to_le_bytes()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//A poll account
#[account]
pub struct Poll {
    pub question: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub poll_index: u64,
    pub creator: PubKey,
}

// vote account, to prove someone has voted on a specific poll
#[account]
pub struct Vote {
    pub poll: Pubkey, //which poll
    pub vote: Pubkey, //Pubkey of voter
}
