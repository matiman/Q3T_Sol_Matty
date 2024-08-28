use anchor_lang::error_code;

#[error_code]
pub enum StakingError {

    #[msg("Maximum stake limit reached")]
    MaxStakeLimitReached,

    #[msg("Frezze period not passed")]
    FreezePeriodNotPassed,
}