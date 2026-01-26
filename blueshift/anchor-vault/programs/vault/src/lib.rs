use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer,transfer};

declare_id!("GWDp2L8CKCMGpdaDVWpJvM3uHRapqASAMn2hSsAjMrLf");
//declare_id!("GWDp2L8CKCMGpdaDVWpJvM3uHRapqASAMn2hSsAjMrLf");
#[program]
pub mod vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require_eq!(vault.lamports() , 0, VaulError::VaultAlreadyExists);

        let rent_fee = Rent::get()?.minimum_balance(0);

        require_gt!(amount , rent_fee, VaulError::InvalidAmount);

        let cpi = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer{
                from :ctx.accounts.signer.to_account_info(),
                to :ctx.accounts.vault.to_account_info(),

            }
        );
        transfer(cpi, amount)
    }

    // pub fn withdraw(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
    //     let signer_key = ctx.accounts.signer.key();
    //     let signer_seeds = &[ SEED, signer_key.as_ref(), &[ ctx.bumps.vault]];
    //     let accounts = Transfer{
    //         from :ctx.accounts.vault.to_account_info(),
    //         to :ctx.accounts.signer.to_account_info(),
    //     };
    //     let cpi = CpiContext::new_with_signer(
    //         ctx.accounts.system_program.to_account_info(), 
    //         accounts, 
    //         &[&signer_seeds[..]]
    //     );
    //     transfer(cpi, ctx.accounts.vault.lamports())
    //   }

    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        require_neq!(ctx.accounts.vault.lamports() , 0, VaulError::InvalidAmount);
        let signer_key = ctx.accounts.signer.key();
        let signer_seeds = &[b"vault", signer_key.as_ref(), &[ctx.bumps.vault]];
        // Transfer all lamports from vault to signer
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.signer.to_account_info(),
                },
                &[&signer_seeds[..]]
            ),
            ctx.accounts.vault.lamports()
        )
    }
}

const SEED : &[u8] = b"vault";
// #[derive(Accounts)]
// pub struct Initialize <'info>{
//     #[account(init, 
//         payer = user, 
//         space = 8  ,
//         seeds = [SEED, user.key.as_ref()],
//         bump,
//     )]
//     pub vault: SystemAccount<'info>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

#[derive(Accounts)]
pub struct VaultAction <'info>{
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [SEED, signer.key.as_ref()], bump)]
    pub vault: SystemAccount<'info>,
        
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaulError{

    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,

//     #[msg("Insufficient balance")]
//     InsufficientBalance,
//     #[msg("Withdraw amount exceeds balance")]
//     WithdrawAmountExceedsBalance,
//     #[msg("Invalid owner")]
//     Invalid_owner,
 }


