use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub bump: u8,
    pub balance: u64,
}

impl Vault {
    pub const LEN: usize = 1 + // bump
        8; // balance
}
