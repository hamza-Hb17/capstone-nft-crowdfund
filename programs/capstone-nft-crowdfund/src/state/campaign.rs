use anchor_lang::prelude::*;

#[account]
pub struct Campaign {
    pub creator: Pubkey,
    pub title: String,
    pub goal: u64,
    pub raised: u64,
    pub deadline: i64,
    pub vault: Pubkey,
    pub description: String,
    pub bump: u8,
}

impl Campaign {
    pub const LEN: usize = 32 + // creator pubkey
        4 + 64 + // title (max 64 chars)
        8 + // goal
        8 + // raised
        8 + // deadline
        32 + // vault pubkey
        4 + 280; // description (max 280 chars)
}
