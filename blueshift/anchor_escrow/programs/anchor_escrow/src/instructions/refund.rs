
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{TokenInterface,TokenAccount,Mint,
                        TransferChecked,transfer_checked,
                        CloseAccount,close_account}
};

use crate::{
    state::{Escrow, ESCROW_SEED},
    errors::EscrowError
};
/**
 *   escro.vault ->maker_ata_a   : token_a
 */
#[derive(Accounts)]
pub struct Refund<'info>{
    #[account(mut)]
    maker : Signer<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [ESCROW_SEED, maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker @EscrowError::InvalidMaker,
        has_one = mint_a @ EscrowError::InvalidMintA,
    
    )]
    pub escrow : Account<'info, Escrow>,
    
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a : InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a : InterfaceAccount<'info, TokenAccount>,
    
    pub associated_token_program : Program<'info, AssociatedToken>,
    pub token_program : Interface<'info , TokenInterface>,
    pub system_program : Program<'info, System>,
}

impl<'info> Refund<'info> {
    // refund_tokens
    fn refund_tokens(&self) -> Result<()> {
        let seeds = &[
            ESCROW_SEED,
            &self.maker.key().to_bytes()[..],
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;


        //close vault
        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        close_account(CpiContext::new_with_signer( 
                    self.associated_token_program.to_account_info(), 
                            close_accounts, 
                            signer_seeds))
    }
}

pub fn handler(ctx: Context<Refund>) -> Result<()> {
    ctx.accounts.refund_tokens()?;
    Ok(())
}