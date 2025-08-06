pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("9xF3XmFUq4RHm8yJm3qZgt3vAKyv3VPZc1W95SYnm7gx");

#[program]
pub mod capstone_nft_crowdfund {
    use super::*;

    pub fn create_campaign(ctx: Context<CreateCampaign>, args: CreateCampaignArgs) -> Result<()> {
        instructions::campaign::create_campaign(ctx, args)
    }

    pub fn contribute_to_campaign(ctx: Context<Contribute>) -> Result<()> {
        instructions::campaign::contribute_to_campaign(ctx)
    }

    pub fn claim_refund(ctx: Context<ClaimRefund>) -> Result<()> {
        instructions::refund::claim_refund(ctx)
    }

    pub fn withdraw_raised_funds(ctx: Context<WithdrawFunds>) -> Result<()> {
        instructions::withdrawal::withdraw_raised_funds(ctx)
    }
}
