use anchor_lang::prelude::*;

#[account]
pub struct Contribution {
    pub contributor: Pubkey,
    pub campaign: Pubkey,
    pub amount: u64,
}

impl Contribution {
    pub const LEN: usize = 32 + 32 + 8;
}
