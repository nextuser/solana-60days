use std::collections::VecDeque;

use anchor_lang::prelude::*;

declare_id!("GmCqJpRBrJsTgXKNvRq8AqwXpjwUJJjDgcpzJdHCUgne");

#[program]
pub mod day16 {
    use std::ops::Deref;
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.my_storage.x = 0xaa;
        ctx.accounts.my_storage.y = 0xbb;
        ctx.accounts.my_storage.z = 0xcc;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn set(ctx:Context<SetStorage>, value : u64) ->Result<()>{
        ctx.accounts.my_storage.x = value * value;
        Ok(())
    }

    pub fn print_x(ctx:Context<ReadStorage>)->Result<()>{
        let s : & MyStorage = ctx.accounts.my_storage.deref();
        msg!("storage x {}, y {}, z {}", s.x,s.y,s.z);
        Ok(())
    }


    pub fn init_flag(ctx:Context<InitFlag>,value :bool)-> Result<()>{
        ctx.accounts.true_or_false.flag = value ;
        msg!("init_flag :{}",value);
        Ok(())
    }

    pub fn set_flag(ctx:Context<SetFlag>,value:bool) -> Result<()>{
        ctx.accounts.true_or_false.flag = value ;
        msg!("set_flag :{}",value);
        Ok(())
    }

    pub fn init_map_data(ctx :Context<MapData>, key : u64,value : u64) -> Result<()> {
        ctx.accounts.val.key = key;
        ctx.accounts.val.value = value;
        msg!("key is {},value is {:?}", key, value);
        Ok(())
    }
    pub fn set_map_data(ctx :Context<SetMapData>, key : u64,value : u64) -> Result<()> {
        ctx.accounts.val.key = key;
        ctx.accounts.val.value = value;
        msg!("set map data : {} => {:?}", key, value);
        Ok(())
    }

    pub fn init_map_info(ctx :Context<InitMapInfo>, key : u64,value : String) -> Result<()> {
        ctx.accounts.info.key = key;
        // ctx.accounts.info.value.copy_from_slice(value.as_bytes());
        copy_from_slice(value.as_bytes(),&mut ctx.accounts.info.value);
        msg!("key is {},value is {:?}", key, value);
        Ok(())
    }
    pub fn set_map_info(ctx :Context<SetMapInfo>, key : u64,value : String) -> Result<()> {
        ctx.accounts.info.key = key;
        // ctx.accounts.info.value.copy_from_slice(value.as_bytes());
        copy_from_slice(value.as_bytes(),&mut ctx.accounts.info.value);
        msg!("set map data : {} => {:?}", key, value);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(init,payer=signer,space=size_of::<MyStorage>() +8,seeds=[b"a"],bump)]
    pub my_storage: Account<'info, MyStorage>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct MyStorage{
    x : u64,
    y : u64,
    z : u64
}

#[derive(Accounts)]
pub struct SetStorage<'info>{
    #[account(mut)]
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct ReadStorage<'info>{
    pub my_storage: Account<'info , MyStorage>
}

#[account]
pub struct TrueOrFalse{
    flag:bool,
}
#[derive(Accounts)]
pub struct SetFlag<'info>{
    #[account(mut)]
    true_or_false:Account<'info,TrueOrFalse>
}

#[derive(Accounts)]
pub struct InitFlag<'info>{
    #[account(mut)]
    signer:Signer<'info>,
    system_program: Program<'info ,System>,
    #[account(init,payer=signer,space=size_of::<TrueOrFalse>()+8, seeds=[b"t"],bump)]
    true_or_false: Account<'info ,TrueOrFalse>,
}


#[account]
pub struct Val{
    pub key : u64,
    pub value : u64,
}

#[derive(Accounts)]
#[instruction(key:u64)]
pub struct MapData<'info>{
    #[account(init,
    payer=signer,
    space=size_of::<Val>() + 8,
    seeds=[&key.to_le_bytes().as_ref()],
    bump)]
    val :Account<'info , Val>,
    #[account(mut)]
    signer:Signer<'info>,
    system_program:Program<'info,System>
}

#[derive(Accounts)]
#[instruction(key:u64)]
pub struct SetMapData<'info>{
    #[account(mut)]
    val :Account<'info , Val>,
}


#[account]
pub struct Info{
    pub key : u64,
    pub value : [u8;32],
}

#[derive(Accounts)]
#[instruction(key:u64)]
pub struct InitMapInfo<'info>{
    #[account(init,
    payer=signer,
    space=size_of::<Info>() + 8,
    seeds=[&key.to_le_bytes().as_ref()],
    bump)]
    info :Account<'info , Info>,
    #[account(mut)]
    signer:Signer<'info>,
    system_program:Program<'info,System>
}

#[derive(Accounts)]
#[instruction(key:u64)]
pub struct SetMapInfo<'info>{
    #[account(mut)]
    info :Account<'info , Info>,
}

fn copy_from_slice(src:&[u8],dst:&mut [u8] ){
    let max = src.len().min(dst.len());
    for i in 0..max{    
        dst[i] = src[i];
    }
}
#[test]
pub fn test_copy_slice(){
    let a :String = String::from("abc");
    let mut value :  [u8;32] = [0;32];
    let bytes = a.as_bytes();
    // let blen = bytes.len();
    // let max = blen.min(bytes.len());

    
    // for i in 0..max{
    //     value[i] = bytes[i];
    // }
    copy_from_slice(bytes,&mut value);
    println!("{:?}",value);
    //std::mem::copy_memory(value.as_mut_ptr(),a.as_ptr(),max);
}

