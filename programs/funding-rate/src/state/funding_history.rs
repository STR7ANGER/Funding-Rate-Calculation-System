use anchor_lang::prelude::*;

#[account]
pub struct FundingHistory {
    pub symbol: String,
    pub timestamp: i64,
    pub funding_rate: i64,
    pub mark_price: u64,
    pub index_price: u64,
    pub total_long_oi: u64, // Open interest
    pub total_short_oi: u64,
}

impl FundingHistory {
    pub const MAX_SIZE: usize = 8 + // discriminator
        4 + 20 + // symbol
        8 + // timestamp
        8 + // funding_rate
        8 + // mark_price
        8 + // index_price
        8 + // total_long_oi
        8; // total_short_oi
}

