use anchor_lang::prelude::*;
use crate::Vault;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Vault::INIT_SPACE,
        // 种子: "vault" + 用户公钥
        seeds = [Vault::SEED_PREFIX, authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let clock = Clock::get()?;

    vault.authority = ctx.accounts.authority.key();
    vault.total_deposits = 0;
    vault.created_at = clock.unix_timestamp;
    vault.bump = ctx.bumps.vault;

    msg!("Vault initialized | Authority: {}", vault.authority);
    Ok(())
}
