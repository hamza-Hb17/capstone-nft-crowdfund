use crate::state::{
    campaign::Campaign, contribution::ContributionAccount, reward_vault::RewardVault, vault::Vault,
};
use anchor_lang::prelude::*;

use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + Campaign::LEN,
        seeds = [b"campaign", creator.key().as_ref()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(
        init,
        payer = creator,
        space = 8 + Vault::LEN,
        seeds = [b"vault", campaign.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = creator,
        space = RewardVault::LEN,
        seeds = [b"reward_vault", campaign.key().as_ref()],
        bump
    )]
    pub reward_vault: Account<'info, RewardVault>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateCampaignArgs {
    pub title: String,
    pub goal: u64,
    pub deadline: i64,
    pub description: String,
}

pub fn create_campaign(ctx: Context<CreateCampaign>, args: CreateCampaignArgs) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    campaign.creator = *ctx.accounts.creator.key;
    campaign.title = args.title;
    campaign.goal = args.goal;
    campaign.deadline = args.deadline;
    campaign.raised = 0;
    campaign.vault = ctx.accounts.vault.key(); // store PDA vault address
    campaign.description = args.description;
    campaign.bump = ctx.bumps.campaign;

    let reward_vault = &mut ctx.accounts.reward_vault;
    reward_vault.campaign = campaign.key();
    reward_vault.total_distributed = 0;
    reward_vault.bump = ctx.bumps.reward_vault;

    Ok(())
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(mut, seeds = [b"campaign", campaign.creator.as_ref()], bump = campaign.bump)]
    pub campaign: Account<'info, Campaign>,

    #[account(
        init_if_needed,
        payer = contributor,
        space = ContributionAccount::LEN,
        seeds = [b"contribution", contributor.key().as_ref(), campaign.key().as_ref()],
        bump
    )]
    pub contribution: Account<'info, ContributionAccount>,

    #[account(
        mut,
        seeds = [b"vault", campaign.key().as_ref()],
        bump,
    )]
    /// CHECK: Vault PDA account
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn contribute_to_campaign(ctx: Context<Contribute>, amount: u64) -> Result<()> {
    // let amount = 1_000_000_000; // 1 SOL

    require!(amount > 0, ErrorCode::InvalidAmount);

    let campaign = &mut ctx.accounts.campaign;
    let contributor = &mut ctx.accounts.contributor;
    let vault = &mut ctx.accounts.vault;
    let contribution = &mut ctx.accounts.contribution;

    // Transfer lamports to vault PDA
    **contributor.to_account_info().try_borrow_mut_lamports()? -= amount;
    **vault.to_account_info().try_borrow_mut_lamports()? += amount;

    // Update campaign state
    campaign.raised += amount;

    // Update user's contribution
    contribution.amount += amount; // Update user's contribution

    contribution.contributor = ctx.accounts.contributor.key();
    contribution.campaign = campaign.key();
    contribution.amount = contribution.amount.checked_add(amount).unwrap();
    contribution.bump = ctx.bumps.contribution;

    Ok(())
}
