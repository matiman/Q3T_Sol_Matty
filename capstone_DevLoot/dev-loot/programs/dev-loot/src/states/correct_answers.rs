use anchor_lang::prelude::*;

//#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
#[account]
pub struct CorrectAnswers{
    pub course_id: u8,
    pub correct_answers: Vec<u8>,//index of questions and answers - 10 questions/course
    pub bump: u8,
    
}

impl Space for CorrectAnswers {

    const INIT_SPACE: usize = 1 + (4 + (1 * 10)) + 1 ; //10 questions/answers
    
}