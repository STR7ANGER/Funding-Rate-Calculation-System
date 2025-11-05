use anchor_lang::prelude::*;

#[error_code]
pub enum FundingRateError {
    #[msg("Invalid funding rate calculation")]
    InvalidCalculation,
    #[msg("Stale price data")]
    StalePrice,
    #[msg("Invalid symbol")]
    InvalidSymbol,
    #[msg("Funding rate out of bounds")]
    RateOutOfBounds,
    #[msg("Position not found")]
    PositionNotFound,
    #[msg("Insufficient margin")]
    InsufficientMargin,
}

