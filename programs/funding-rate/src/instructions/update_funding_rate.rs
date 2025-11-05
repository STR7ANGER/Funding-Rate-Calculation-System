use anchor_lang::prelude::*;
use crate::state::funding_rate_state::FundingRateState;
use crate::utils::math::{calculate_premium_index, calculate_funding_rate};
use crate::constants::INTEREST_RATE_DAILY_BPS;

#[derive(Accounts)]
pub struct UpdateFundingRate<'info> {
    #[account(mut)]
    pub funding_state: Account<'info, FundingRateState>,
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateFundingRate>, mark_price: u64, index_price: u64) -> Result<()> {
    let funding_state = &mut ctx.accounts.funding_state;
    
    // Calculate premium index
    let premium_index = calculate_premium_index(mark_price, index_price)?;
    
    // Calculate funding rate
    let funding_rate = calculate_funding_rate(premium_index, INTEREST_RATE_DAILY_BPS)?;
    
    // Update state
    funding_state.mark_price = mark_price;
    funding_state.index_price = index_price;
    funding_state.premium_index = premium_index;
    funding_state.current_rate = funding_rate;
    funding_state.last_update = Clock::get()?.unix_timestamp;
    
    // Add to hourly samples
    funding_state.add_sample(funding_rate);
    
    Ok(())
}

