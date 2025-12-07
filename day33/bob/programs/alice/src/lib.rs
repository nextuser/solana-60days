use anchor_lang::prelude::*;
use bob::cpi::accounts::BobAddOp;
use bob::program::Bob;
use bob::BobData;
declare_id!("GiTXosnBXHYjH5ukfqTCzTA6m3AHwAko4dnFUAxcbid9");

#[program]
pub mod alice {
    use super::*;
    pub fn ask_bob_to_add(ctx:Context<AliceOp>,
                            a:u64,
                            b:u64)->Result<()>{
        let cpi_program = ctx.accounts.bob_program.to_account_info();
        let cpi_account = BobAddOp{
            bob_data_account : ctx.accounts.bob_data_account.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program,cpi_account);

        let res = bob::cpi::add_and_store(cpi_ctx, a, b);
        if res.is_ok(){
            Ok(())
        } else{
            err!(Errors::CpiToBobFailed)
        }
    }
}

#[derive(Accounts)]
pub struct AliceOp <'info>{
    #[account(mut)]
    pub bob_data_account : Account<'info,BobData>,
    pub bob_program : Program<'info,Bob>,

}

#[error_code]
pub enum Errors{
    #[msg("cpi to bob failed")]
    CpiToBobFailed,
}
