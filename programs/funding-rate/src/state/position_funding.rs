use anchor_lang::prelude::*;

#[account]
pub struct PositionFunding {
    pub position_pubkey: Pubkey,
    pub symbol: String,
    pub position_size: u64,
    pub last_funding_timestamp: i64,
    pub accrued_funding: i64, // Signed: positive = received, negative = paid
}

impl PositionFunding {
    pub const MAX_SIZE: usize = 8 + // discriminator
        32 + // position_pubkey
        4 + 20 + // symbol
        8 + // position_size
        8 + // last_funding_timestamp
        8; // accrued_funding
}

