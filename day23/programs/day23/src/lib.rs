use anchor_lang::prelude::*;
use anchor_lang::system_program::{self,Transfer};

declare_id!("7PKAbFgWnFrrW1DQWrjVu1daefSxgJqCwA1utHitmkWk");

#[program]
pub mod day23 {
    use super::*;

    pub fn transfer_sol(ctx: Context<TrasferAccounts>,amount :u64) -> Result<()> {
        let from = &ctx.accounts.from;
        let to = &ctx.accounts.to;
        let system_program = &ctx.accounts.system_program;

        let cpi_context = CpiContext::new(system_program.to_account_info(), Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
        });
        let res = system_program::transfer(cpi_context, amount);
        if res.is_ok() {
            msg!("Transfer successful {} => {} :{}",from.key(), to.key(), amount);
        } else {
            msg!("Transfer failed");
        }

        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    /**
    'a: 关联账户信息(account info)的生命周期
    'b: 程序账户(program account)的生命周期
    'c: 剩余账户(remaining accounts)的生命周期
    'info: 整个上下文结构体的生命周期
    */
    pub fn split_sol<'a,'b,'c,'info> (ctx:Context<'a,'b,'c,'info,SplitSol<'info>>, amount : u64)
                ->Result<()>{

        let amount_each_gas = amount /(ctx.remaining_accounts.len() as u64);
        let the_program = &ctx.accounts.system_program;
        for recipient in  ctx.remaining_accounts {
            let cpi_accounts = system_program::Transfer{
                from : ctx.accounts.signer.to_account_info(),
                to : recipient.to_account_info(),
            };
            let cpi_program = the_program.to_account_info();
            let cpi_context = CpiContext::new(cpi_program,cpi_accounts);
            let res = system_program::transfer(cpi_context,amount_each_gas);
            if !res.is_ok(){
                return err!(Errors::TransferFailed)
            }
        }
        Ok(())
    }
}

#[error_code]
pub enum Errors{
    #[msg("transfer failed")]
    TransferFailed,
}

#[derive(Accounts)]
pub struct SplitSol<'info>{
    #[account(mut)]
    signer:Signer<'info>,
    system_program:Program<'info,System>
}

#[derive(Accounts)]
pub struct TrasferAccounts <'info>{
    ///CHECK :
    #[account(mut)]
    pub from: Signer<'info>,
    ///CHECK :
    #[account(mut)]
    pub to: UncheckedAccount<'info>,
    pub system_program : Program<'info, System>,
}
