use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{CourseConfig, Student, StudentPrgress};

#[derive(Accounts)]
#[instruction(wallet: Pubkey)]
pub struct EnrollStudent<'info>{

    #[account(mut)]
    pub student: Signer<'info> , //TODO allow only admins to add students ??

    #[account(
        init,
        payer = student,
        space = 8 + Student::INIT_SPACE,
        seeds = [b"student".as_ref(), wallet.key().as_ref()], //wallet is unique for each student
        bump,
    )]
    pub student_account: Box<Account<'info, Student>>,

    #[account(
        init,
        payer = student,
        space = 8 + StudentPrgress::INIT_SPACE,
        seeds = [
            b"student_progress".as_ref(), 
            wallet.key().as_ref(),
            course_config.key().as_ref()],//student can take multiple courses so have course config
        bump,
    )]
    pub student_progress: Box<Account<'info, StudentPrgress>>,

    #[account(
        seeds = [b"course_config".as_ref(), &[course_config.course_id]],
        bump
    )]
    pub course_config: Box<Account<'info,CourseConfig>>,

    pub system_program: Program<'info, System>,

}

impl<'info> EnrollStudent<'info> {

    pub fn enroll_student(&mut self, wallet: Pubkey, full_name: String,
         is_paid_student: bool,bumps: &EnrollStudentBumps)-> Result<()>{
        let now =  Clock::get()?.unix_timestamp;

        if is_paid_student {
            self.deduct_sol()?;
        }

        self.student_account.set_inner( Student{
            wallet,
            full_name,
            is_paid_student,
            bump: bumps.student_account
        });

        self.student_progress.set_inner( StudentPrgress { 
            course_id: self.course_config.course_id,
            content_at: 0,
            total_points_earned: 0,
            course_completed: false,
            last_updated: now,
            registered_at: now,
            staking_points_earned: 0,
            amount_staked: 0,
            bump: bumps.student_progress
             }
        );

        Ok(())
    }

    //If students are paid, deduct Sol from their account.
    pub fn deduct_sol(&mut self)-> Result<()>{

        let cpi_program= self.system_program.to_account_info();
        //later on we will use this amount to buy them memecoins.
        let cpi_accounts = Transfer{
            from: self.student.to_account_info(),
            to: self.student_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        //TODO student paying 0.001 SOL
        transfer(cpi_ctx, 1_000_000)?;
        
        Ok(())

    }

}