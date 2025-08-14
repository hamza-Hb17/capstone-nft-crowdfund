use anchor_lang::prelude::*;

#[account]
pub struct RewardVault {
    pub campaign: Pubkey,       // Linked campaign
    pub total_distributed: u64, // Total amount ever distributed
    pub bump: u8,
}

impl RewardVault {
    pub const LEN: usize = 32 + 8 + 1;
}
