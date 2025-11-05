use anchor_lang::prelude::*;

declare_id!("KNFMTjh5SgpfbsddFDNKao4FGwwr13DhWBDHXmuE7h2");

pub mod instructions;
pub mod state;
pub mod errors;
pub mod constants;
pub mod utils;

use instructions::*;

#[program]
pub mod funding_rate {
    use super::*;

    pub fn initialize_funding_state(
        ctx: Context<InitializeFundingState>,
        symbol: String,
    ) -> Result<()> {
        instructions::initialize_funding_state::handler(ctx, symbol)
    }

    pub fn update_funding_rate(
        ctx: Context<UpdateFundingRate>,
        mark_price: u64,
        index_price: u64,
    ) -> Result<()> {
        instructions::update_funding_rate::handler(ctx, mark_price, index_price)
    }

    pub fn apply_hourly_funding(
        ctx: Context<ApplyHourlyFunding>,
    ) -> Result<()> {
        instructions::apply_hourly_funding::handler(ctx)
    }
}

