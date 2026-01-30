use anchor_lang::prelude::*;
mod errors;
mod instructions;
mod state;
use instructions::*;

//declare_id!("GHNy6w8h5wJXLgbgxbh2bqmYDGMJukfivGQw6whRGhiV");
declare_id!("22222222222222222222222222222222222222222222");
#[program]
pub mod anchor_escrow {
    use super::*;
    pub fn make(ctx: Context<Make>,seed:u64,receive:u64, amount : u64) -> Result<()> {
        make::handler(ctx, seed, receive, amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        take::handler(ctx)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        refund::handler(ctx)
    }


}


#[derive(Accounts)]
pub struct Initialize {}



