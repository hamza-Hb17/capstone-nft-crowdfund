use crate::state::{Campaign, Vault};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut, has_one = creator)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub vault: Account<'info, Vault>,
}

pub fn withdraw_raised_funds(ctx: Context<WithdrawFunds>) -> Result<()> {
    // Placeholder for withdrawal logic
    Ok(())
}
