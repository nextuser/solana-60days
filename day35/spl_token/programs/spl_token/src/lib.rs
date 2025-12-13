use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
declare_id!("4CCTDw86fsubQZNNVnUhVGr2NytmQupShaxnhabyHa6Q");

#[program]
pub mod spl_token {
    use super::*;
    pub fn create_and_mint_token(ctx:Context<CreateMint>,token_name:String)->Result<()>{
        let mint_amonunt = 100_000_000_000;
        let mint = ctx.accounts.new_mint.clone();
        let dest_ata = &ctx.accounts.new_ata;
        let token_program = ctx.accounts.token_program.clone();
        // mint accont context
        let mint_to_instruction = token::MintTo{
            mint:mint.to_account_info(),
            to: dest_ata.to_account_info(),
            authority: ctx.accounts.signer.clone().to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            token_program.to_account_info(), mint_to_instruction);
        token::mint_to(cpi_ctx, mint_amonunt)?;

        Ok(())
        
    }


}

#[derive(Accounts)]
#[instruction(token_name: String)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = signer,
        mint::freeze_authority = signer,
        seeds=[signer.key().as_ref(),token_name.as_bytes()],
        bump)]
    pub new_mint : Account<'info,Mint>,
    #[account(init ,
        payer = signer,
        associated_token::authority = signer,
        associated_token::mint = new_mint
    )]   
    pub new_ata:Account<'info,TokenAccount>,

    pub token_program: Program<'info,Token>,
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub system_program: Program<'info,System>,
}
