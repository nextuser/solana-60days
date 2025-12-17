use anchor_lang::prelude::*;

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
       ctx.accounts.user_account.amount = 0;
       msg!("create user account: {:?} , authority: {:?}",
                ctx.accounts.user_account.key(),
                ctx.accounts.user.key() );
       
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(
        init,
        payer=payer,
        space=std::mem::size_of::<Bank>()+8)
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
    pub amount : u64,
}
