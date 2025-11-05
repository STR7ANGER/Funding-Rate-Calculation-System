use anchor_lang::prelude::*;
use crate::constants::SECONDS_PER_HOUR;

#[account]
pub struct FundingRateState {
    pub symbol: String, // max 20 chars
    pub current_rate: i64, // Signed, basis points (10000 = 1%)
    pub mark_price: u64,
    pub index_price: u64,
    pub premium_index: i64,
    pub interest_rate: i64,
    pub last_update: i64,
    pub hourly_samples: [i64; 3600], // Store 1 hour of 1-second samples
    pub sample_index: u16,
    pub authority: Pubkey,
}

impl FundingRateState {
    pub const MAX_SIZE: usize = 8 + // discriminator
        4 + 20 + // symbol (String)
        8 + // current_rate (i64)
        8 + // mark_price (u64)
        8 + // index_price (u64)
        8 + // premium_index (i64)
        8 + // interest_rate (i64)
        8 + // last_update (i64)
        3600 * 8 + // hourly_samples
        2 + // sample_index (u16)
        32; // authority (Pubkey)

    pub fn add_sample(&mut self, rate: i64) {
        self.hourly_samples[self.sample_index as usize] = rate;
        self.sample_index = (self.sample_index + 1) % SECONDS_PER_HOUR;
    }

    pub fn get_hourly_average(&self) -> i64 {
        let sum: i128 = self.hourly_samples.iter().map(|&x| x as i128).sum();
        (sum / SECONDS_PER_HOUR as i128) as i64
    }
}

