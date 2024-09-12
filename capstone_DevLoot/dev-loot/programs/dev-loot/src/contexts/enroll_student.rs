use anchor_lang::prelude::*;

use crate::{CourseConfig, Student, StudentPrgress};

#[derive(Accounts)]
#[instruction(wallet: String)]
pub struct EnrollStudent<'info>{

    #[account(mut)]
    pub student: Signer<'info> , //TODO allow only admins to add students ??

    #[account(
        init,
        payer = student,
        space = 8 + Student::INIT_SPACE,
        seeds = [b"student".as_ref(), wallet.as_str().as_ref()], //wallet is unique for each student
        bump,
    )]
    pub student_account: Account<'info, Student>,

    #[account(
        init,
        payer = student,
        space = 8 + StudentPrgress::INIT_SPACE,
        seeds = [
            b"student_progress".as_ref(), 
            wallet.as_str().as_ref(),
            course_config.key().as_ref()],//student can take multiple courses so have course config
        bump,
    )]
    pub student_progress: Account<'info, StudentPrgress>,

    #[account(
        seeds = [b"course_config".as_ref(), &[course_config.course_id]],
        bump
    )]
    pub course_config: Account<'info,CourseConfig>,

    pub system_program: Program<'info, System>,

}

impl<'info> EnrollStudent<'info> {

    pub fn enroll_student(&mut self, wallet: Pubkey, full_name: String, bumps: &EnrollStudentBumps,)-> Result<()>{
        let now =  Clock::get()?.unix_timestamp;

        self.student_account.set_inner( Student{
            wallet,
            full_name,
            bump: bumps.student_account
        });

        self.student_progress.set_inner( StudentPrgress { 
            course_id: self.course_config.course_id,
            content_at: 0,
            total_points_earned: 0,
            course_completed: false,
            last_updated: now,
            registered_at: now,
            bump: bumps.student_progress
             }
        );

        Ok(())
    }

}