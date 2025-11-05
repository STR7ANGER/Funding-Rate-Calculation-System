use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::FundingRateError;

pub fn calculate_premium_index(mark_price: u64, index_price: u64) -> Result<i64> {
    require!(index_price > 0, FundingRateError::InvalidCalculation);
    let premium = ((mark_price as i128 - index_price as i128) * BASIS_POINT_SCALE as i128) / index_price as i128;
    Ok(premium as i64)
}

pub fn calculate_funding_rate(premium_index: i64, interest_rate_bps: i64) -> Result<i64> {
    let rate = premium_index + interest_rate_bps;
    let clamped = rate.max(FUNDING_RATE_MIN_BPS).min(FUNDING_RATE_MAX_BPS);
    Ok(clamped)
}

pub fn calculate_funding_payment(position_size: u64, avg_funding_rate_bps: i64) -> Result<i64> {
    let payment = (position_size as i128 * avg_funding_rate_bps as i128) / BASIS_POINT_SCALE as i128;
    Ok(payment as i64)
}

