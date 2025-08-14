pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::{campaign, claim_rewards, nft_mint, refund, withdrawal};

declare_id!("9xF3XmFUq4RHm8yJm3qZgt3vAKyv3VPZc1W95SYnm7gx");

#[program]
pub mod defi_nft_crowdfunding {
    use super::*;

    pub fn create_campaign(
        ctx: Context<campaign::CreateCampaign>,
        args: campaign::CreateCampaignArgs,
    ) -> Result<()> {
        campaign::create_campaign(ctx, args)
    }

    pub fn contribute_to_campaign(ctx: Context<campaign::Contribute>) -> Result<()> {
        campaign::contribute_to_campaign(ctx)
    }

    pub fn claim_refund(ctx: Context<refund::ClaimRefund>) -> Result<()> {
        refund::claim_refund(ctx)
    }

    pub fn withdraw_raised_funds(ctx: Context<withdrawal::WithdrawFunds>) -> Result<()> {
        withdrawal::withdraw_raised_funds(ctx)
    }

    pub fn mint_nft(ctx: Context<nft_mint::MintNft>) -> Result<()> {
        nft_mint::mint_nft(ctx)
    }

    pub fn claim_rewards(ctx: Context<claim_rewards::ClaimRewards>) -> Result<()> {
        claim_rewards::handler(ctx)
    }
}
