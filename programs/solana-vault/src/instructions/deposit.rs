use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{error::ErrorCode, Vault};


#[derive(Accounts)]
pub struct Deposit<'info>{
    
    #[account(
        mut,
        seeds=[Vault::SEED_PREFIX, vault.authority.as_ref()],
        bump=vault.bump,
        has_one = authority//constrain vault.authority == authority.key()
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,


}



pub fn handle_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    
    require!(amount > 0, ErrorCode::InvalidDepositAmount);

    let cpi_accounts = Transfer {
        from: ctx.accounts.authority.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.key(),
        cpi_accounts,
    );
    transfer(cpi_context, amount)?;


    let vault = &mut ctx.accounts.vault;

    vault.total_deposits = vault
    .total_deposits
    .checked_add(amount)
    .ok_or(ErrorCode::Overflow)?;

    msg!("Deposited {} lamports into the vault | Total Deposits: {}", amount, vault.total_deposits);

    Ok(())
}