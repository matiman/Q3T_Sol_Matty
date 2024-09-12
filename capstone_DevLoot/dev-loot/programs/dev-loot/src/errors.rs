use anchor_lang::error_code;

#[error_code]
pub enum StudentError {

    #[msg("Can't reward a student who hasn't completed course yet!")]
    CourseNotCompleted,

    #[msg("Student progress can't be backward!")]
    CantProgressBackward,

    #[msg("Student can only earn positive score!")]
    CanOnlyEarnPositiveScore,

    #[msg("You can only progress a course you havent completed!")]
    CantUpdateCompletedCourse
    
}