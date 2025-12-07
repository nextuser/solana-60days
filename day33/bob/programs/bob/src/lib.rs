use anchor_lang::prelude::*;

declare_id!("3fthK9MEzNJWGkTMQha7qQnSJHKPJ3DVLfB4raKKcSH9");

#[program]
pub mod bob {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn add_and_store(ctx:Context<BobAddOp> , a:u64,b:u64)->Result<()>{
        let data = &mut ctx.accounts.bob_data_account ;
        let result = a.checked_add(b);
        match result {
            Some(value)=>{
                data.result = value;
            } ,
            None=>{
                return Err(ErrorCode::Overflow.into());
            }
        }
        
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow")]
    Overflow,
}

#[account]
pub struct BobData{
    pub result : u64,
}


#[derive(Accounts)]
pub struct BobAddOp<'info> {
    #[account(mut)]
    pub bob_data_account: Account<'info, BobData>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + size_of::<BobData>())]
    pub bob_data_account: Account<'info, BobData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}