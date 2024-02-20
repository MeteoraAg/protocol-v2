use anchor_lang::prelude::*;

pub mod controller;
pub mod error;
pub mod instructions;
pub mod macros;
pub mod math;
pub mod state;

use instructions::*;

#[cfg(feature = "mainnet-beta")]
declare_id!("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");
#[cfg(not(feature = "mainnet-beta"))]
declare_id!("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");

#[program]
pub mod drift {
    use super::*;

    #[allow(unused)]
    pub fn initialize_user_stats(ctx: Context<InitializeUserStats>) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    pub fn initialize_user(
        ctx: Context<InitializeUser>,
        sub_account_id: u16,
        name: [u8; 32],
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    pub fn update_spot_market_cumulative_interest(
        ctx: Context<UpdateSpotMarketCumulativeInterest>,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    pub fn deposit(
        ctx: Context<Deposit>,
        market_index: u16,
        amount: u64,
        reduce_only: bool,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    pub fn withdraw(
        ctx: Context<Withdraw>,
        market_index: u16,
        amount: u64,
        reduce_only: bool,
    ) -> Result<()> {
        Ok(())
    }
}
