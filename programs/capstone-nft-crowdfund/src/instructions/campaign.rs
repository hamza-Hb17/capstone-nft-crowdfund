use crate::state::{campaign::Campaign, vault::Vault};
use anchor_lang::prelude::*;

use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + Campaign::LEN,
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub creator: Signer<'info>,

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
}

pub fn create_campaign(ctx: Context<CreateCampaign>, args: CreateCampaignArgs) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    campaign.creator = *ctx.accounts.creator.key;
    campaign.title = args.title;
    campaign.goal = args.goal;
    campaign.deadline = args.deadline;
    campaign.raised = 0;
    campaign.vault = ctx.accounts.vault.key();

    Ok(())
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(mut)]
    pub campaign: Account<'info, Campaign>,

    /// CHECK: We validate vault PDA manually
    #[account(mut)]
    pub vault_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub system_program: Program<'info, System>,
}

pub fn contribute_to_campaign(ctx: Context<Contribute>, amount: u64) -> Result<()> {
    // let amount = 1_000_000_000; // 1 SOL

    require!(amount > 0, ErrorCode::InvalidAmount);

    // Transfer SOL to the vault PDA
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.contributor.key(),
        &ctx.accounts.vault_account.key(),
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.contributor.to_account_info(),
            ctx.accounts.vault_account.to_account_info(),
        ],
    )?;

    // Update campaign state
    ctx.accounts.campaign.raised += amount;

    Ok(())
}
