use anchor_lang::prelude::*;

declare_id!("EPrHGoFbK5Fz9MwLwZjxS86FZP78dkREzbioTNm4Q2EH");

#[program]
pub mod an5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize2>, a:u64,b:u64,info:String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("you get a={} b={}, infor={}",a,b,info);
        Ok(())
    }

    pub fn array(_ctx: Context<Initialize2>, v : Vec<String>)-> Result<()> {
        msg!("array: {:?} ,\n size={}", v,v.len());
        Ok(())
    }




    pub fn add(_ctx: Context<Initialize2>, a:u8,b:u8)-> Result<()> {
        let  v : u8 = a.checked_add(b).unwrap();
        msg!("add value: {}",&v);
        Ok(())
    }

    pub fn test_overflow(_ctx: Context<Initialize2>, )-> Result<()> {
        let mut v : u8 = 255;
        v = v.checked_add(2).unwrap();
        msg!("add value: {}",&v);
        Ok(())
    }

    pub fn cbrt(_ctx: Context<Initialize2>, f : f32)-> Result<()> {
        msg!("cbrt: {}",f32::cbrt(f));
         Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize2 {}
