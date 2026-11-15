use anchor_lang::prelude::*;
use crate::{error::ErrorCode, Vault};


#[derive(Accounts)]
pub struct Withdraw<'info> {

    #[account(
        mut,
        seeds=[Vault::SEED_PREFIX, vault.authority.as_ref()],
        bump=vault.bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub authority: Signer<'info>,

}

pub fn handle_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {

    require!(amount > 0, ErrorCode::InvalidWithdrawAmount);
    let vault_info = ctx.accounts.vault.to_account_info();
    let authority_info = ctx.accounts.authority.to_account_info();

    let rent = Rent::get()?;
    let min_balance = rent.minimum_balance(
        vault_info.data_len()
    );

    let current_lamports = vault_info.lamports();

    let available_amount = current_lamports
        .checked_sub(min_balance)
        .unwrap_or(0);

    let vault = &mut ctx.accounts.vault;

    require!(
        amount <= vault.total_deposits,
        ErrorCode::InsufficientBalance
    );

    require!(
        amount <= available_amount,
        ErrorCode::InsufficientBalance
    );

    let new_vault_balance = current_lamports
        .checked_sub(amount)
        .ok_or(ErrorCode::InsufficientBalance)?;

    let new_authority_balance = authority_info
        .lamports()
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;

    **vault_info.try_borrow_mut_lamports()? = new_vault_balance;

    **authority_info.try_borrow_mut_lamports()? = new_authority_balance;

    vault.total_deposits = vault.total_deposits
        .checked_sub(amount)
        .ok_or(ErrorCode::InsufficientBalance)?;

    msg!("Withdrew {} lamports from the vault | Total Deposits: {}", amount, vault.total_deposits);

    
    Ok(())
}
