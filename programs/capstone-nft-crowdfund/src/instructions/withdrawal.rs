use crate::state::campaign::Campaign;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut, address = campaign.creator)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub campaign: Account<'info, Campaign>,

    #[account(
        mut,
        seeds = [b"vault", campaign.key().as_ref()],
        bump,
    )]
    /// CHECK: Vault PDA account
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_funds(ctx: Context<WithdrawFunds>) -> Result<()> {
    let campaign = &ctx.accounts.campaign;
    let vault = &mut ctx.accounts.vault;
    let creator = &mut ctx.accounts.creator;

    let amount = campaign.raised;

    // Transfer lamports from vault to creator
    **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **creator.to_account_info().try_borrow_mut_lamports()? += amount;

    Ok(())
}
