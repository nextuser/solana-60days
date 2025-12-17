use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer,Transfer}; 

declare_id!("G4cjBGgrkYBvErabZ5b4XMvqHL56vFqB6txn9u7FR181");

#[program]
pub mod basic_bank {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.bank.manager = ctx.accounts.payer.key();
        ctx.accounts.bank.total_deposit = 0;
        msg!("bank initialize bank config: {:?} , manager: {:?}",
                ctx.accounts.bank.to_account_info().key().to_string(),
                ctx.accounts.payer.key().to_string());
        Ok(())
    }

    pub fn create_user_account(ctx: Context<CreaeUserAccount>) -> Result<()> {
       ctx.accounts.user_account.authority = ctx.accounts.user.key();
       ctx.accounts.user_account.balance = 0;
       msg!("create user account: {:?} , authority: {:?}",
                ctx.accounts.user_account.key(),
                ctx.accounts.user.key() );
       
        Ok(())
    }

    pub fn deposit(ctx:Context<Deposit>,amount:u64) -> Result<()>{ 
        require!(amount > 0 , BankError::InvalidAmount);

        let user = &ctx.accounts.user.key();
        let bank = &ctx.accounts.bank.key();
        let transfer_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.bank.to_account_info(),
            },
        );
        transfer(transfer_ctx, amount)?;

        let user_account = &mut ctx.accounts.user_account;
        user_account.balance = user_account.balance.checked_add(amount).ok_or(BankError::Overflow)?;

        let bank = &mut ctx.accounts.bank;
        bank.total_deposit = bank.total_deposit.checked_add(amount).ok_or(BankError::Overflow)?;

        msg!("deposit success:  user_account: {:?} deposit lamports: {:?}, new balance : {:?} , bank total{}",
                user_account.key(),
                amount,
                user_account.balance,
                bank.total_deposit
                );
        Ok(())

    }

    pub fn withdraw(ctx:Context<Withdraw>,amount:u64) -> Result<()>{ 
        require!(ctx.accounts.user_account.authority == ctx.accounts.user.key(), BankError::InvalidAuthority);
        require!(ctx.accounts.user_account.balance >= amount, BankError::InsufficientBalance);
        require!(ctx.accounts.bank.total_deposit >= amount, BankError::InsufficientDeposit);

        // let cpi_ctx = CpiContext::new(
        //     ctx.accounts.system_program.to_account_info(),
        //     Transfer {
        //         from: ctx.accounts.bank.to_account_info(),
        //         to: ctx.accounts.user.to_account_info(),
        //     },
        // );
        // transfer(cpi_ctx, amount)?;
        let bank_account = ctx.accounts.bank.to_account_info();
        let new_deposit = bank_account.lamports().checked_sub(amount).ok_or(BankError::Overflow)?;

        ** bank_account.to_account_info().try_borrow_mut_lamports()? = new_deposit;

        let user_account_info = ctx.accounts.user.to_account_info();
        let new_user_balance =  user_account_info.lamports().checked_add(amount).ok_or(BankError::Overflow)?;
        ** user_account_info.try_borrow_mut_lamports()? = new_user_balance;

        msg!("withdraw success:  user_account: {:?} withdraw lamports: {:?}, new balance : {:?} , bank total deposit: {:?}",
                ctx.accounts.user_account.key(),
                amount,
                ctx.accounts.user_account.balance,
                ctx.accounts.bank.total_deposit
                );
        Ok(())
    }

    



}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UseAccount>,
    #[account(mut)]
    bank: Account<'info, Bank>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum BankError {
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Overflow")]
    Overflow,
    #[msg("Invalid Authority")]
    InvalidAuthority,
    #[msg("Insufficient Balance")]
    InsufficientBalance,
    #[msg("Insufficient Deposit")]
    InsufficientDeposit,
}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(
        init,
        payer=payer,
        space=std::mem::size_of::<Bank>()+8,
    )
    ]
    pub bank :Account<'info, Bank>,
    #[account(mut)]
    pub payer:Signer<'info>,
    pub system_program:Program<'info, System>,

}


#[account]
pub struct Bank {
    manager: Pubkey,
    pub total_deposit: u64,
}

#[derive(Accounts)]
pub struct CreaeUserAccount<'info> {
    #[account(
        init,
        payer=payer,
        space=std::mem::size_of::<UseAccount>()+8,
        seeds=[b"bank_account", user.key().as_ref()],
        bump
    )
    ]
    user_account: Account<'info, UseAccount>,
    #[account(mut)]
    payer   : Signer<'info>,
    /// CHECK:
    user: AccountInfo<'info>,
    pub system_program:Program<'info, System>,
}

#[account]
pub struct UseAccount{
    pub authority: Pubkey,
    pub balance : u64,
}



#[derive(Accounts)]

pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UseAccount>,
    #[account(mut)]
    bank: Account<'info, Bank>,
    pub system_program: Program<'info, System>,
}
