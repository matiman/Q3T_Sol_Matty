use anchor_lang::error_code;

#[error_code]
pub enum StudentError {

    #[msg("Can't reward a student who hasn't completed course yet!")]
    CourseNotCompleted
    
}