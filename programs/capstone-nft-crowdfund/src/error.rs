use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid contribution amount.")]
    InvalidAmount,
    #[msg("Campaign is still active.")]
    CampaignStillActive,
    #[msg("Campaign succeeded. No refund.")]
    CampaignSucceeded,
    #[msg("No refundable contribution found.")]
    NoRefundAvailable,
}
