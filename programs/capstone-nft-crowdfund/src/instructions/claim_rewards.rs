use crate::state::{CampaignAccount, ClaimRecord, ContributionAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(
        mut,
        seeds = [b"campaign", campaign.id.as_ref()],
        bump = campaign.bump
    )]
    pub campaign: Account<'info, CampaignAccount>,

    #[account(
        mut,
        seeds = [b"contribution", campaign.key().as_ref(), contributor.key().as_ref()],
        bump
    )]
    pub contribution: Account<'info, ContributionAccount>,

    #[account(
        mut,
        seeds = [b"claim", campaign.key().as_ref(), contributor.key().as_ref()],
        bump
    )]
    pub claim_record: Account<'info, ClaimRecord>,

    #[account(mut)]
    /// CHECK: lamport vault account
    pub reward_vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClaimRewards>) -> Result<()> {
    let total_raised = ctx.accounts.campaign.raised;
    let reward_pool = ctx.accounts.reward_vault.to_account_info().lamports();
    let contribution = ctx.accounts.contribution.amount;

    let claimable = reward_pool
        .checked_mul(contribution)
        .unwrap()
        .checked_div(raised)
        .unwrap();

    let already_claimed = ctx.accounts.claim_record.claimed_amount;
    let to_claim = claimable.checked_sub(already_claimed).unwrap();

    **ctx
        .accounts
        .reward_vault
        .to_account_info()
        .try_borrow_mut_lamports()? -= to_claim;
    **ctx
        .accounts
        .contributor
        .to_account_info()
        .try_borrow_mut_lamports()? += to_claim;

    ctx.accounts.claim_record.claimed_amount += to_claim;

    Ok(())
}
