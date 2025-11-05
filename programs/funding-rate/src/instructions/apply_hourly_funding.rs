use anchor_lang::prelude::*;
use crate::state::{funding_rate_state::FundingRateState, position_funding::PositionFunding};
use crate::utils::math::calculate_funding_payment;

#[derive(Accounts)]
pub struct ApplyHourlyFunding<'info> {
    #[account(mut)]
    pub funding_state: Account<'info, FundingRateState>,
    #[account(mut)]
    pub position_funding: Account<'info, PositionFunding>,
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<ApplyHourlyFunding>) -> Result<()> {
    let funding_state = &ctx.accounts.funding_state;
    let position_funding = &mut ctx.accounts.position_funding;
    
    // Get hourly average funding rate
    let avg_funding_rate = funding_state.get_hourly_average();
    
    // Calculate funding payment
    let payment = calculate_funding_payment(position_funding.position_size, avg_funding_rate)?;
    
    // Update position funding
    position_funding.accrued_funding = position_funding.accrued_funding
        .checked_add(payment)
        .ok_or(crate::errors::FundingRateError::InvalidCalculation)?;
    position_funding.last_funding_timestamp = Clock::get()?.unix_timestamp;
    
    // Emit event would go here
    // emit!(FundingPaymentEvent { ... });
    
    Ok(())
}

