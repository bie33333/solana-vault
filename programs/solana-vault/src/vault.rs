use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    /// Vault Owner
    pub authority: Pubkey,
    /// Total Deposits (Logical Accounting)
    pub total_deposits: u64,
    /// Creation Timestamp
    pub created_at: i64,
    /// PDA Bump (Store it to save subsequent calculations)
    pub bump: u8,
}

impl Vault {
    pub const SEED_PREFIX: &'static [u8] = b"vault";
}