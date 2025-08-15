use crate::state::{Campaign, RewardVault};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SendRevenue<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(seeds = [b"campaign", campaign.creator.as_ref()], bump = campaign.bump)]
    pub campaign: Account<'info, Campaign>,

    #[account(
        mut,
        seeds = [b"reward_vault", campaign.key().as_ref()],
        bump = reward_vault.bump
    )]
    pub reward_vault: Account<'info, RewardVault>,

    #[account(mut)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn send_revenue(ctx: Context<SendRevenue>, amount: u64) -> Result<()> {
    // update state
    let reward_vault = &mut ctx.accounts.reward_vault;
    reward_vault.total_distributed = reward_vault.total_distributed.checked_add(amount).unwrap();

    // transfer SOL to the reward vault
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.creator.key(),
        &ctx.accounts.vault.key(), // or a vault PDA
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.creator.to_account_info(),
            ctx.accounts.vault.to_account_info(),
        ],
    )?;

    Ok(())
}
