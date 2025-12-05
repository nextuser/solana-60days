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
