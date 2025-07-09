use anchor_lang::prelude::*;

declare_id!("4Y5yssNBE2pCiKtWYahKJ97LoXEo9gi7bbGRm635Mpj7");

#[program]
pub mod simple_voting_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
