use anchor_lang::prelude::*;
use std::mem::size_of;
declare_id!("3XeWiF3X8trEBrDajh3xVL3r48V3r3ARFDSLcd5K73U8");

#[program]
pub mod day24 {
    use super::*;

    pub fn initialize<'a,'b,'c,'info>(ctx: Context<'a,'b,'c,'info,Initialize<'info>>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(init,payer=fren,space=size_of::<MyStorage>()+8, seeds=[b"a"], bump)]
    pub my_storage: Account<'info,MyStorage>,
    #[account(mut)]
    pub fren :Signer<'info>,
    pub system_program:Program<'info ,System>
}

#[account]
pub struct MyStorage{
    x : u64
}
