use anchor_lang::prelude::*;
use crate::Vault;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub vault: Account<'info, Vault>,

    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {

    Ok(())
}
