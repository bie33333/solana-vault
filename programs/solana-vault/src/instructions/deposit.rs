use anchor_lang::prelude::*;

use crate::{constants::*, error::ErrorCode, Vault};

#[derive(Accounts)]
pub struct Deposit<'info>{
    
    pub vault: Account<'info, Vault>,

    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,


}



pub fn handle_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    
    Ok(())
}