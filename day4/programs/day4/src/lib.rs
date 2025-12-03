use anchor_lang::prelude::*;

declare_id!("ADmDHn5SffLaf6Y4MH838w3kNqx5zwXmLwUHhPtUsG2a");

#[program]
pub mod day4 {
    use super::*;

    pub fn initialize(ctx: Context<LimitRange>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn limit_range(ctx:Context<LimitRange>, a: u64) -> Result<()> {
        if a < 10 {
            return err!(MyError::AisTooSmall)
        } else if(a > 100) {
            return err!(MyError::AisBig)
        }
        msg!("limit_range call ,a is {}", a);
        Ok(())
    }

    pub fn sayHello(ctx :Context<LimitRange> ) -> Result<()> {
        msg!("sayHello call from {}", ctx.program_id);
        Ok(())
    }
}

#[error_code]
pub enum MyError{
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("a is big")]
    AisBig,

}

#[derive(Accounts)]
pub struct LimitRange {}
