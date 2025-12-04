use anchor_lang::prelude::*;

declare_id!("3akvGAsKkioGy5T6VjA9pSNwWk6hsAUokPb4HMXPpq4Z");

#[program]
pub mod day13_event {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        emit!(MyEvent{value : 42});
        emit!(MySecondEvent{value : 3, message:"hello evevent of second".to_string()});
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[event]
pub struct MyEvent{
    pub value : u64,
}

#[event]
pub struct MySecondEvent{
    pub value : u64,
    pub message : String,
}
