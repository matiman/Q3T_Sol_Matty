use anchor_lang::prelude::*;

use crate::{Student, StudentPrgress};

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
        seeds = [b"student_progress".as_ref(), student_account.wallet.key().as_ref()],
        bump = student_progress.bump
    )]
    pub student_progress: Account<'info, StudentPrgress>,

}

impl<'info> UpdateStudentProgress<'info>{

    //to update all student progress fields at once. 
    //Use other methods to update a specific progress like score.
    pub fn bulk_update_student_progress(&mut self, 
        course_id: u8,
        content_at: u8,
        new_points_earned: u8,
        course_completed: bool ) -> Result<()> {

            self.student_progress.course_id = course_id;
            self.student_progress.content_at = content_at;
            self.student_progress.total_points_earned += new_points_earned; //TODO Error check here for negative
            self.student_progress.course_completed = course_completed;
            self.student_progress.last_updated = Clock::get()?.unix_timestamp;

            Ok(())
    }

    pub fn update_score(&mut self, new_points_earned: u8, new_content_index: u8) -> Result<()> {

        self.student_progress.total_points_earned += new_points_earned;
        self.student_progress.content_at = new_content_index;
                
        Ok(())

    }

    //this is to update student content_at.
    //it can be used if users miss to answer questions 2x. No need to update score.

    pub fn update_content_pointer(&mut self, new_content_index: u8) -> Result<()> {
    
        self.student_progress.content_at = new_content_index;
               
        Ok(())

    }

    pub fn complete_course(&mut self) -> Result<()> {
    
        self.student_progress.course_completed =true;
               
        Ok(())

    }

}