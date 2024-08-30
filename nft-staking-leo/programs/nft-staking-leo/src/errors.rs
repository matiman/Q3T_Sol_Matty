use anchor_lang::prelude::*;


///TODO: Abstract out.
#[error_code]
pub enum StakingError {
    #[msg("Already staked")]
    AlreadyStaked,

    #[msg("Attributes not initialized")]
    AttributesNotInitialized,

    #[msg("Invalid Timestamp")]
    InvalidTimeStamp,

    #[msg("Asset Not Staked.")]
    NotStaked,

    #[msg("Arthimetic Overflow.")]
    Overflow,

    #[msg("Arthimetic Underflow.")]
    Underflow,
}