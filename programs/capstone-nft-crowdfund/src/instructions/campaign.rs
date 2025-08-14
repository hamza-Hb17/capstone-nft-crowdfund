use crate::state::{campaign::Campaign, vault::Vault};
use anchor_lang::prelude::*;

use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + CampaignAccount::MAX_SIZE,
        seeds = [b"campaign", creator.key().as_ref(), args.title.as_bytes()],
        bump
    )]
    pub campaign: Account<'info, CampaignAccount>,

    #[account(
        seeds = [b"vault", campaign.key().as_ref()],
        bump,
        payer = creator,
        space = 8 + Vault::LEN
    )]
    pub vault: Account<'info, Vault>,

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
    campaign.vault = ctx.accounts.vault.key(); // ✅ store PDA vault address
    campaign.description = args.description;

    Ok(())
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(mut)]
    pub campaign: Account<'info, CampaignAccount>,

    #[account(
        init_if_needed,
        payer = contributor,
        space = 8 + ContributionAccount::SIZE,
        seeds = [b"contribution", campaign.key().as_ref(), contributor.key().as_ref()],
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

    // Transfer lamports to vault PDA
    **contributor.to_account_info().try_borrow_mut_lamports()? -= amount;
    **vault.to_account_info().try_borrow_mut_lamports()? += amount;

    // Update campaign state
    ctx.accounts.campaign.raised += amount;

    // Update user's contribution
    ctx.accounts.contribution.amount += amount; // Update user's contribution

    Ok(())
}
