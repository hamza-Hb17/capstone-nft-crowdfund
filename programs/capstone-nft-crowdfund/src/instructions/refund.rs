use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimRefund<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(mut)]
    pub campaign: Account<'info, CampaignAccount>,

    #[account(
        mut,
        seeds = [b"vault", campaign.key().as_ref()],
        bump
    )]
    /// CHECK: Vault PDA
    pub vault: UncheckedAccount<'info>,

    #[account(mut, seeds = [b"contribution", campaign.key().as_ref(), contributor.key().as_ref()], bump)]
    pub contribution: Account<'info, ContributionAccount>,

    pub system_program: Program<'info, System>,
}

pub fn claim_refund(ctx: Context<ClaimRefund>) -> Result<()> {
    let campaign = &ctx.accounts.campaign;
    let clock = Clock::get()?;

    // check camping is still active
    require!(
        clock.unix_timestamp > campaign.deadline,
        CustomError::CampaignStillActive
    );
    // check if the compaing get the goal
    require!(
        campaign.raised < campaign.goal,
        CustomError::CampaignSucceeded
    );

    let amount = ctx.accounts.contribution.amount; // stored earlier per contributor
    require!(amount > 0, CustomError::NoRefundAvailable);

    // Transfer lamports from vault to contributor
    **ctx
        .accounts
        .vault
        .to_account_info()
        .try_borrow_mut_lamports()? -= amount;
    **ctx
        .accounts
        .contributor
        .to_account_info()
        .try_borrow_mut_lamports()? += amount;

    // set Zero to avoid double refund
    ctx.accounts.contribution.amount = 0;

    Ok(())
}
