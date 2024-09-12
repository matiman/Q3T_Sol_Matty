use anchor_lang::prelude::*;

use crate::{CourseConfig, Student, StudentError, StudentPrgress};

#[derive(Accounts)]
pub struct UpdateStudentProgress<'info>{

    #[account(mut)]
    pub student: Signer<'info>,

    //just to derive the wallet but its not mutable at this point
    #[account(
        seeds = [b"student".as_ref(), student_account.wallet.key().as_ref()],
        bump = student_account.bump
    )]
    pub student_account: Account<'info, Student>,

    #[account(
        mut,
        seeds = [
            b"student_progress".as_ref(), 
            student_account.wallet.key().as_ref(),
            course_config.key().as_ref()],//student can take multiple courses so have course config
        bump = student_progress.bump
    )]
    pub student_progress: Account<'info, StudentPrgress>,

    #[account(
        seeds = [b"course_config".as_ref(),&[course_config.course_id]],
        bump
    )]
    pub course_config: Account<'info,CourseConfig>,

}

impl<'info> UpdateStudentProgress<'info>{

    //to update all student progress fields at once. 
    //Use other methods to update a specific progress like score.
    pub fn bulk_update_student_progress(&mut self, 
        content_at: u8,
        new_points_earned: u8 ) -> Result<()> {

            require!(self.student_progress.course_completed == false, StudentError::CantUpdateCompletedCourse);
            require!(content_at > self.student_progress.content_at, StudentError::CantProgressBackward);
            require!(new_points_earned > 0, StudentError::CanOnlyEarnPositiveScore);

            self.student_progress.content_at = content_at;
            self.student_progress.total_points_earned += new_points_earned; //TODO Error check here for negative
            self.student_progress.course_completed = self.is_at_last_index_of_course();
            self.student_progress.last_updated = Clock::get()?.unix_timestamp;

            //TODO handle error
            if self.is_at_last_index_of_course(){
                let _= self.complete_course();
            }

            Ok(())
    }

    pub fn update_score(&mut self, new_points_earned: u8, content_at: u8) -> Result<()> {

        require!(self.student_progress.course_completed == false, StudentError::CantUpdateCompletedCourse);
        require!(content_at > self.student_progress.content_at, StudentError::CantProgressBackward);
        require!(new_points_earned > 0, StudentError::CanOnlyEarnPositiveScore);

        self.student_progress.total_points_earned += new_points_earned;
        self.student_progress.content_at = content_at;

        if self.is_at_last_index_of_course(){
            let _ = self.complete_course();
        }
         
        Ok(())
    }

    //this is to update student content_at.
    //it can be used if users miss to answer questions 2x. No need to update score.

    pub fn update_content_pointer(&mut self, content_at: u8) -> Result<()> {
    
        require!(self.student_progress.course_completed == false, StudentError::CantUpdateCompletedCourse);
        require!(content_at > self.student_progress.content_at, StudentError::CantProgressBackward);
        
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
        if self.student_progress.content_at>= self.course_config.last_content_index
        {
            true
        }
       else {
        false
       }
        
    }

}