use anchor_lang::prelude::*;

declare_id!("9xF3XmFUq4RHm8yJm3qZgt3vAKyv3VPZc1W95SYnm7gx");

#[program]
pub mod capstone_nft_crowdfund {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
