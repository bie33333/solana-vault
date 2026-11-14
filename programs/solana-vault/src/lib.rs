pub mod constants;
pub mod error;
pub mod instructions;
pub mod vault;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use vault::*;

declare_id!("7dr2VvHqdD6KWzeU7etbxhzCSsy4wYbD3s9TMHbJ55j9");

#[program]
pub mod solana_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        crate::instructions::deposit::handle_deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()>{
        crate::instructions::withdraw::handle_withdraw(ctx, amount)
    }

}
