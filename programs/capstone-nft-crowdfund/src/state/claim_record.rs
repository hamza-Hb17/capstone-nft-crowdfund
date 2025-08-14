use anchor_lang::prelude::*;

#[account]
pub struct ClaimRecord {
    pub contributor: Pubkey,
    pub campaign: Pubkey,
    pub claimed_amount: u64,
    pub bump: u8,
}

impl ClaimRecord {
    pub const LEN: usize = 32 + 32 + 8 + 1;
}
