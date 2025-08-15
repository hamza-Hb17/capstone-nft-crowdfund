use anchor_lang::prelude::*;

#[account]
pub struct ContributionAccount {
    pub contributor: Pubkey,
    pub campaign: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

impl ContributionAccount {
    pub const LEN: usize = 32 + 32 + 8 + 1;
}
