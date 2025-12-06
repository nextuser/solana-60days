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


    pub fn init_player(ctx:Context<InitPlayer>) -> Result<()>{
        ctx.accounts.player.authority = ctx.accounts.signer.key();
        ctx.accounts.player.points = 10000;
        msg!("init player");
        Ok(())
    }

    pub fn transfer_points(ctx:Context<TrasferPoints>,amount:u64) -> Result<()>{
        // require!(ctx.accounts.from.authority == ctx.accounts.authority.key(), Errors::SignerNotAuthorized);
        // require!(amount <= ctx.accounts.from.points, Errors::InsurfficientPoints);
        ctx.accounts.from.points -= amount;
        ctx.accounts.to.points += amount;
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

#[error_code]
pub enum Errors{
    #[msg("Not enough points")]
    InsurfficientPoints,
    #[msg("Signer is Not Authorized")]
    SignerNotAuthorized,
}
    


#[account]
pub struct Player{
    pub authority: Pubkey,
    pub points: u64,
}

#[derive(Accounts)]
pub struct InitPlayer<'info>{
    #[account(init,
        payer=signer,
        space=size_of::<Player>() + 8,
        seeds=[&signer.as_ref().key().to_bytes()],
        bump)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}   
#[derive(Accounts)]
#[instruction(amount:u64)]
pub struct TrasferPoints<'info>{
    #[account(mut,has_one=authority, constraint = from.points >= amount)]
    from : Account<'info,Player>,
    #[account(mut)]
    to : Account<'info,Player>,
    authority: Signer<'info>,
}


