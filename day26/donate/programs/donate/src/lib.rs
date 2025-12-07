use anchor_lang::prelude::*;
use anchor_lang::system_program;


declare_id!("ETidF73gevL8auZkgoF2fi7PpydEKpZ9ZE1n175HGQfy");

#[program]
pub mod donate {

    use super::*;

    pub fn donate(ctx:Context<Donate>, amount: u64) ->Result<()>{
        let system_program_account = ctx.accounts.system_program.to_account_info();
        let system_program_id = system_program::ID;
        //如果没有初始化,其实init_if_needed 会真的初始化完成
         if ctx.accounts.pda.to_account_info().data_is_empty() ||
            ctx.accounts.pda.withdrawer == system_program_id
        {
            ctx.accounts.pda.withdrawer = * ctx.accounts.withdrawer.key;
            msg!("donate init withrawer.key: {},pda.withdray={}", ctx.accounts.withdrawer.key.to_string(),ctx.accounts.pda.withdrawer.to_string());
          } else {
            
            require!(ctx.accounts.pda.withdrawer == *ctx.accounts.withdrawer.key, ErrorCodes::MismatchWithdrawer);
         }

        let tansfer_accounts  =             anchor_lang::system_program::Transfer{
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.pda.to_account_info(),
            };



        let cpi_context = CpiContext::new(
            system_program_account,tansfer_accounts
        );
        system_program::transfer(cpi_context, amount)?;
        msg!("Donated: {} => {} : {}; wait {} to claim",
            ctx.accounts.signer.key().to_string(),
            ctx.accounts.pda.key().to_string(),
            amount,
            ctx.accounts.withdrawer.key().to_string(),
        );
        Ok(())
    }

    pub fn withdraw(ctx:Context<Withdraw>,amount: u64) ->Result<()>{
        if *ctx.accounts.withdrawer.key != ctx.accounts.pda.withdrawer {
            msg!("Error:withrawer.key = {},but expect pda.withdrawer={}" ,ctx.accounts.withdrawer.key.to_string(), ctx.accounts.pda.withdrawer.to_string());
            return Ok(());
        }
        require!(amount <= ctx.accounts.pda.get_lamports(), ErrorCodes::Insufficient);
        ctx.accounts.pda.sub_lamports(amount)?;
        ctx.accounts.withdrawer.add_lamports(amount)?;
        msg!("withdraw {} by {}", amount, ctx.accounts.withdrawer.key.key().to_string());
        
        Ok(())
    
    }
}

#[error_code]
pub enum ErrorCodes{
    #[msg("the signer is not allowed to withdraw")]
    MismatchWithdrawer,
    #[msg("insufficient balance to  withdraw")]
    Insufficient,
    #[msg("doante mismatch withdrawer")]
    DonateMismatchWithdrawer,
}
#[account]
pub struct Pda {
    withdrawer: Pubkey
}

#[derive(Accounts)]
pub struct Donate <'info>{
    //注意必须在Cargo.toml 增加 features = ["init-if-needed"]
    #[account(
        init_if_needed,
        payer = signer,
        space = size_of::<Pda>() + 8,
        seeds = [withdrawer.key.as_array()],
        bump
    )]
    pub pda: Account<'info, Pda >,
    /// CHECK: this is an account asociated with the pda
    pub withdrawer : AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct Donate<'info>{
//     #[account(mut)]
//     pub signer: Signer<'info>,
//     #[account(mut)]
//     pub pda : Account<'info, Pda>,
//     pub withdrawer: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
//
// }

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    #[account(mut)]
    pub pda : Account<'info, Pda>,
    //pub system_program: Program<'info, System>,
}
