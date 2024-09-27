use anchor_lang::prelude::*;

pub mod contexts;
pub mod states;
pub mod errors;

pub use contexts::*;
pub use states::*;
pub use errors::*;

declare_id!("DGpFWc2R2aMZc8rF9jSBuqBrtnQ7VdD8nCuTwJ6vNp66");

#[program]
pub mod dev_loot {

    use super::*;

    pub fn init_config(ctx: Context<InitConfig>, course_id: u8, last_content_index: u8,
                        total_questions: u8, min_points_for_reward:u8 ) -> Result<()> {
        ctx.accounts.initialize_config(course_id, last_content_index,
                                         total_questions,min_points_for_reward, &ctx.bumps)?;
        Ok(())
    }

    pub fn enroll_free_student(ctx: Context<EnrollStudent>, wallet: Pubkey,
         full_name: String) -> Result<()> {

        ctx.accounts.enroll_student(wallet, full_name, false, &ctx.bumps)?;
        Ok(())
    }

    pub fn enroll_paid_student(ctx: Context<EnrollStudent>, wallet: Pubkey,
        full_name: String) -> Result<()> {

       ctx.accounts.enroll_student(wallet, full_name, true, &ctx.bumps)?;
       Ok(())
   }

    pub fn bulk_update_student_progress(ctx: Context<UpdateStudentProgress>, content_at: u8,
        new_points_earned: u8 ) -> Result<()> {

        ctx.accounts.bulk_update_student_progress(content_at, new_points_earned)?;
        Ok(())
    }

    pub fn update_score(ctx: Context<UpdateStudentProgress>, new_points_earned: u8, new_content_index: u8, ) -> Result<()> {
            
        ctx.accounts.update_score(new_points_earned, new_content_index)?;
        Ok(())
    }

    pub fn update_content_pointer(ctx: Context<UpdateStudentProgress>, new_content_index:u8 ) -> Result<()> {
            
        ctx.accounts.update_content_pointer(new_content_index)?;
        Ok(())
    }

    //TODO this should be called automatically once student reaches final index of course.
    pub fn complete_course(ctx: Context<UpdateStudentProgress> ) -> Result<()> {
            
        ctx.accounts.complete_course()?;
        Ok(())
    }

    //TODO combine this with course completion and make it automatic staking
    pub fn stake_diamond_nft(ctx: Context<StakeDiamondNft> ) -> Result<()> { 
        ctx.accounts.stake_diamond_nft(&ctx.bumps)?;
        Ok(())
    }


     pub fn unstake_diamond_nft(ctx: Context<UnstakeDiamondNft> ) -> Result<()> { 
        ctx.accounts.unstake()?;
        Ok(())
    }

    pub fn claim_staking_rewards(ctx: Context<ClaimStakingRewards> ) -> Result<()> { 
        ctx.accounts.claim()?;
        Ok(())
    }

    
}


