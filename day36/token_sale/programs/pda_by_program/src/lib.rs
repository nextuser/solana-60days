use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer,CreateAccount,create_account};

declare_id!("7UduVVa9aMtcdg2dFzkY1TeGGwyJQTN22AUBqVzyYveZ");

#[program]
pub mod pda_by_program {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let rent = Rent::get()?;
        let rent_reserve = rent.minimum_balance(0);
        //let (pda,bump) = Pubkey::find_program_address(&[b"pda_account"], &ctx.program_id);
        let  cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
            CreateAccount {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.pda_account.to_account_info(),
            });
        create_account(cpi_context, 
            0,//size
            rent_reserve,
            &System::id()//rent
        )?;
        msg!("system create pda account {}, program:{}", 
        ctx.accounts.pda_account.key.to_string(), ctx.program_id.to_string());

        Ok(())
    }


    pub fn sol_transfer(ctx: Context<SolTransfer>,amount :u64) -> Result<()> {
        let from_pubkey = ctx.accounts.pda_account.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let bump_seed = ctx.bumps.pda_account;
        let signer_seeds:&[&[&[u8]]] = &[&[b"pda_account",&[bump_seed]]];
        
        let cpi_context = CpiContext::new(
            program_id, 
            Transfer{
                from:from_pubkey,
                to:to_pubkey,
            }
        ).with_signer(signer_seeds);

        transfer(cpi_context,amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolTransfer <'info>{
    payer:Signer<'info>,
    /// CHECK:
    #[account(
        mut,
        seeds=[b"pda_account"]
        ,bump)]
    pda_account: AccountInfo<'info>,
    #[account(mut)]
    recipient:SystemAccount <'info>,
    system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct  Initialize<'info>{
    #[account(mut)]
    payer:Signer<'info>,
    /// CHECK:
    #[account(mut,
        seeds=[b"pda_account"],
        bump,
    )]
    pda_account: AccountInfo<'info>,
    system_program: Program<'info, System>,
}