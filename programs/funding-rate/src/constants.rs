use anchor_lang::constant;

pub const MAX_SYMBOL_LENGTH: usize = 20;
pub const INTEREST_RATE_DAILY_BPS: i64 = 1; // 0.01% = 1 basis point
pub const FUNDING_RATE_MIN_BPS: i64 = -50; // -0.05%
pub const FUNDING_RATE_MAX_BPS: i64 = 50; // 0.05%
pub const SECONDS_PER_HOUR: u16 = 3600;
pub const BASIS_POINT_SCALE: i64 = 10000;

