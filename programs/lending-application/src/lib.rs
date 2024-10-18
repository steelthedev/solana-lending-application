use anchor_lang::prelude::*;

use instructions::*;

declare_id!("8jcuB4FXSjDVHiYv4d8wKovR9NbwQqygoHzeUzGW8RSv");

mod constants;
mod error;
mod instructions;
mod state;

#[program]
pub mod lending_application {
    use super::*;

    pub fn init_bank(
        ctx: Context<InitBank>,
        liquidation_threshold: u64,
        max_ltv: u64,
    ) -> Result<()> {
        let _ = process_init_bank(ctx, liquidation_threshold, max_ltv);
        Ok(())
    }

    pub fn init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
        let _ = process_init_user(ctx, usdc_address);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let _ = process_deposit(ctx, amount);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let _ = process_withdraw(ctx, amount);
        Ok(())
    }
}
