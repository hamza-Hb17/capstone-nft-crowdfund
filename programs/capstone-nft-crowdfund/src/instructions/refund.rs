use crate::state::Campaign;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimRefund<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
}

pub fn claim_refund(ctx: Context<ClaimRefund>) -> Result<()> {
    // Placeholder for refund logic
    Ok(())
}
