
use anchor_lang::prelude::*;
use anchor_spl::associated_token::{AssociatedToken};
use anchor_spl::token_interface::{TokenInterface,TokenAccount,Mint,TransferChecked,transfer_checked};
use crate::{ 
    state::{Escrow,ESCROW_SEED},
    errors::EscrowError
};

/**
 * 
 *  maker -> maker_ata_a  : token_a
 *  maker wait for :  token_b (amount)
 *
 */

 #[test]
 fn test_space_of_discriminator(){
    assert_eq!(Escrow::DISCRIMINATOR.len(), 1);
 }

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Make<'info>{
    #[account(mut)]
    pub maker : Signer<'info>,
    #[account(
        init,
        payer = maker,
        space = Escrow::INIT_SPACE + Escrow::DISCRIMINATOR.len(),
        seeds = [ESCROW_SEED, maker.key().as_ref(), &seed.to_le_bytes()],
        bump
    )]
    pub escrow : Account<'info, Escrow>,

    
    #[account(mint::token_program = token_program)]
    pub mint_a : InterfaceAccount<'info, Mint>,
    
    #[account(mint::token_program = token_program)]
    pub mint_b : InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program, 
    )]

    pub maker_ata_a : InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault :InterfaceAccount<'info, TokenAccount>,

    // 使用token的常用program
    pub associated_token_program : Program<'info, AssociatedToken>,
    pub token_program : Interface<'info, TokenInterface>,
    pub system_program : Program<'info, System>,

}


impl<'info> Make<'info>{
    fn populate_escrow(&mut self,seed :u64, receive :u64, bump:u8) {
        self.escrow.set_inner(Escrow{
            seed : seed,
            maker :  self.maker.key(),
            mint_a : self.mint_a.key(),
            mint_b : self.mint_b.key(),
            receive : receive,
            bump : bump,
        });

    }   


    fn deposit_tokens(&self,amount : u64) -> Result<()>{
        let cpi_accounts = TransferChecked{
            from : self.maker_ata_a.to_account_info(),
            to : self.vault.to_account_info(),
            authority : self.maker.to_account_info(),
            mint : self.mint_a.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)
    }
}


pub fn handler(ctx:Context<Make>,seed : u64, deposit:u64, receive : u64) -> Result<()>{ 
    require_gt!(receive,0,EscrowError::InvalidAmount);
    require_gt!(receive , 0,EscrowError::InvalidAmount);
    ctx.accounts.populate_escrow(seed,receive,ctx.bumps.escrow);
    ctx.accounts.deposit_tokens(deposit)?;
    Ok(())
}