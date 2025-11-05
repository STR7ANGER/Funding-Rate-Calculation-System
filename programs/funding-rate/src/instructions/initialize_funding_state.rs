use anchor_lang::prelude::*;
use crate::state::funding_rate_state::FundingRateState;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(symbol: String)]
pub struct InitializeFundingState<'info> {
    #[account(
        init,
        payer = authority,
        space = FundingRateState::MAX_SIZE,
        seeds = [b"funding_rate", symbol.as_bytes()],
        bump
    )]
    pub funding_state: Account<'info, FundingRateState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeFundingState>, symbol: String) -> Result<()> {
    require!(symbol.len() <= MAX_SYMBOL_LENGTH, crate::errors::FundingRateError::InvalidSymbol);
    
    let funding_state = &mut ctx.accounts.funding_state;
    funding_state.symbol = symbol;
    funding_state.current_rate = 0;
    funding_state.mark_price = 0;
    funding_state.index_price = 0;
    funding_state.premium_index = 0;
    funding_state.interest_rate = INTEREST_RATE_DAILY_BPS;
    funding_state.last_update = Clock::get()?.unix_timestamp;
    funding_state.hourly_samples = [0; 3600];
    funding_state.sample_index = 0;
    funding_state.authority = ctx.accounts.authority.key();
    
    Ok(())
}

