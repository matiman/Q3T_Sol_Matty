use anchor_lang::prelude::*;

use crate::{Student, StudentPrgress};

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
        seeds = [b"student_progress".as_ref(), wallet.as_str().as_ref()],//TODO change to use student.wallet instead?.
        bump,
    )]
    pub student_progress: Account<'info, StudentPrgress>,

    pub system_program: Program<'info, System>,

    // #[account(
    //     init,
    //     payer = admin,
    //     space = 8 + StudentRewards::INIT_SPACE,
    //     seeds = [b"student_rewards".as_ref(), wallet.as_str().as_ref()],
    //     bump,
    // )]
    // pub student_rewards: Account<'info, StudentRewards>,

    //pub rewards_mint: Account<'info, Vec<RewardType>>, //TODO use vector for mints

    // pub diamond_mint: InterfaceAccount<'info, Mint>, //our mint for students who score > 80%
    // pub gold_mint: InterfaceAccount<'info, Mint>, //our mint for those < 80%

    // #[account(
    //     init_if_needed,
    //     payer = admin,
    //     associated_token::mint = diamond_mint,
    //     associated_token::authority = student,
    // )]
    // pub student_diamond_ata: InterfaceAccount<'info, TokenAccount>,

    // #[account(
    //     init_if_needed,
    //     payer = admin,
    //     associated_token::mint = gold_mint,
    //     associated_token::authority = student,
    // )]
    // pub student_gold_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> EnrollStudent<'info> {

    pub fn enroll_student(&mut self, wallet: String, full_name: String, bumps: &EnrollStudentBumps,
    course_id: u8)-> Result<()>{
        let now =  Clock::get()?.unix_timestamp;

        self.student_account.set_inner( Student{
            wallet,
            full_name,
            bump: bumps.student_account
        });

        self.student_progress.set_inner( StudentPrgress { 
            course_id,
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