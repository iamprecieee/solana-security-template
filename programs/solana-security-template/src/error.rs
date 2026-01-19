use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Source and destination accounts cannot be the same")]
    DuplicateAccount,

    #[msg("Unauthorized - signer does not match authority")]
    Unauthorized,

    #[msg("Invalid data format")]
    InvalidData,

    #[msg("Arithmetic overflow")]
    Overflow,
}
