use anchor_lang::error_code;

#[error_code]
pub enum DevLootErrorCodes {

    #[msg("Student can only earn positive score!")]
    CanOnlyEarnPositiveScore,

    #[msg("Student progress can't be backward!")]
    CantProgressBackward,

    #[msg("You can only progress a course you havent completed!")]
    CantUpdateCompletedCourse,

    #[msg("Can't reward a student who hasn't completed course yet!")]
    CourseNotCompleted,

    #[msg("Answer index must be between 0 and 3")]
    InvalidAnswerIndex,

    #[msg("The answer the student provided is incorrect")]
    IncorrectAnswer,


    #[msg("Maximum stake limit reached")]
    MaxStakeLimitReached,

    #[msg("Frezze period not passed")]
    FreezePeriodNotPassed,
    
}