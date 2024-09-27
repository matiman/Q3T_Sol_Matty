use anchor_lang::prelude::*;

use crate::{CourseConfig, DevLootErrorCodes, Student, StudentPrgress};

#[derive(Accounts)]
pub struct UpdateStudentProgress<'info>{

    #[account(mut)]
    pub student: Signer<'info>,

    //just to derive the wallet but its not mutable at this point
    #[account(
        seeds = [b"student".as_ref(), student_account.wallet.key().as_ref()],
        bump = student_account.bump
    )]
    pub student_account: Box<Account<'info, Student>>,

    #[account(
        mut,
        seeds = [
            b"student_progress".as_ref(), 
            student_account.wallet.key().as_ref(),
            course_config.key().as_ref()],//student can take multiple courses so have course config
        bump = student_progress.bump
    )]
    pub student_progress: Box<Account<'info, StudentPrgress>>,

    #[account(
        seeds = [b"course_config".as_ref(),&[course_config.course_id]],
        bump
    )]
    pub course_config: Box<Account<'info,CourseConfig>>,

    // #[account(
    //     seeds = [b"correct_answers", course_config.key().as_ref() ],
    //     bump = answers.bump
    // )]
    // pub answers: Box<Account<'info,CorrectAnswers>>,
}

impl<'info> UpdateStudentProgress<'info>{

    //to update all student progress fields at once. 
    //Use other methods to update a specific progress like score.
    pub fn bulk_update_student_progress(&mut self, 
        content_at: u8,
        new_points_earned: u8 ) -> Result<()> {

            require!(self.student_progress.course_completed == false, DevLootErrorCodes::CantUpdateCompletedCourse);
            require!(content_at > self.student_progress.content_at, DevLootErrorCodes::CantProgressBackward);
            require!(new_points_earned > 0, DevLootErrorCodes::CanOnlyEarnPositiveScore);

            self.student_progress.content_at = content_at;
    
            self.student_progress.total_points_earned = self.student_progress.total_points_earned.checked_add(new_points_earned).
            ok_or(ProgramError::ArithmeticOverflow)?;

            self.student_progress.course_completed = self.is_at_last_index_of_course();
            self.student_progress.last_updated = Clock::get()?.unix_timestamp;

            //TODO handle error
            if self.is_at_last_index_of_course(){
                let _= self.complete_course();
            }

            Ok(())
    }

    pub fn update_score(&mut self, new_points_earned: u8, content_at: u8) -> Result<()> {

        require!(self.student_progress.course_completed == false, DevLootErrorCodes::CantUpdateCompletedCourse);
        require!(content_at > self.student_progress.content_at, DevLootErrorCodes::CantProgressBackward);
        require!(new_points_earned > 0, DevLootErrorCodes::CanOnlyEarnPositiveScore);

        self.student_progress.total_points_earned = self.student_progress.total_points_earned.checked_add(new_points_earned).
        ok_or(ProgramError::ArithmeticOverflow)?;
        self.student_progress.content_at = content_at;

        if self.is_at_last_index_of_course(){
            let _ = self.complete_course();
        }
         
        Ok(())
    }

    //this is to update student content_at.
    //it can be used if users miss to answer questions 2x. No need to update score.

    pub fn update_content_pointer(&mut self, content_at: u8) -> Result<()> {
    
        require!(self.student_progress.course_completed == false, DevLootErrorCodes::CantUpdateCompletedCourse);
        require!(content_at > self.student_progress.content_at, DevLootErrorCodes::CantProgressBackward);
        
        self.student_progress.content_at = content_at;

        if self.is_at_last_index_of_course(){
            let _= self.complete_course();
        }
               
        Ok(())

    }

    pub fn complete_course(&mut self) -> Result<()> {
        self.student_progress.course_completed =true;
               
        Ok(())

    }

    pub fn is_at_last_index_of_course(&mut self) -> bool {
        self.student_progress.content_at>= self.course_config.last_content_index

    }

    // //Verify if students input is correct.
    // pub fn is_correct_answer(&mut self, question_id:usize, student_answer_id:u8,
    // first_trial: bool) -> Result<bool> {

    //     let student_answer = Some(student_answer_id);
    //     let correct_answer = self.answers.correct_answers.get(question_id);

    //     // if student_answer.unwrap().eq(correct_answer){
    //     //     Ok(false)

    //     // }

    //     Ok(true)
    //     //require!(self.answers.correct_answers.get(question_id).unwrap()== &answer_id,DevLootErrorCodesIncorrectAnswer);            

    // }

}