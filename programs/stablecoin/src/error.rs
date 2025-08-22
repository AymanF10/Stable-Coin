use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid Price")]
    InvalidPrice,

    #[msg("Below Minimum Health Factor")]
    BelowMinHealthFactor,

    #[msg("Cannot liquidate a healthy account")]
    AboveMinHealthFactor,
}