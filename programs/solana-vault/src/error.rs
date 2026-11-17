use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {


    #[msg("Deposit amount must be greater than zero")]
    InvalidDepositAmount,

    #[msg("#Withdraw amount must be greater than zero")]
    InvalidWithdrawAmount,

    #[msg("Insufficient balance in the vault")]
    InsufficientBalance,

    #[msg("Overflow occurred during the operation")]
    Overflow,

    
}
