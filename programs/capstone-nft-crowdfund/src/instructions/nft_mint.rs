use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction()]
pub struct MintNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub campaign: Account<'info, CampaignAccount>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
        seeds = [b"nft_mint", campaign.key().as_ref(), payer.key().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK: Only used as signer
    #[account(seeds = [b"mint_authority"], bump)]
    pub mint_authority: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {
    let mint_authority_seeds: &[&[u8]] = &[b"mint_authority", &[ctx.bumps.mint_authority]];

    let signer_seeds = &[mint_authority_seeds];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        },
        signer_seeds,
    );

    token::mint_to(cpi_ctx, 1)?;
    Ok(())
}
