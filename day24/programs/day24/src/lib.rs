use anchor_lang::prelude::*;
use std::mem::size_of;
use bs58;
declare_id!("3XeWiF3X8trEBrDajh3xVL3r48V3r3ARFDSLcd5K73U8");

#[program]
pub mod day24 {
    use super::*;

    pub fn initialize<'a,'b,'c,'info>(ctx: Context<'a,'b,'c,'info,Initialize<'info>>) -> Result<()> {
        msg!("day24 initialize: by {:?} programId={}",bs58::encode(&ctx.accounts.fren.key).into_string(),ctx.program_id);
        Ok(())
    }

    pub fn update<'info>(ctx:Context<UpldateValues<'info>>,value:u64) ->Result<()>{
        ctx.accounts.my_storage.x = value;
        msg!("day24 update: by {:?} x={}",bs58::encode(&ctx.accounts.fren.key).into_string(),value);
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

#[derive(Accounts)]
pub struct UpldateValues<'info>{
    #[account(mut ,seeds=[b"a"], bump)]
    pub my_storage: Account<'info,MyStorage>,
    #[account(mut)]
    pub fren : Signer<'info>

}