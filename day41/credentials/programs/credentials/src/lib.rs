use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token, token_2022_extensions::{NonTransferableMintInitialize, non_transferable_mint_initialize}, token_interface::{Mint, Token, TokenAccount, TokenInterface, mint_to}
};

declare_id!("HRnacqBRyEkqobxXwX6W1tjZYFmcHNetZvNkWbgEYerf");

#[program]
pub mod credentials {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        non_transferable_mint_initialize(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
        NonTransferableMintInitialize {
            mint: ctx.accounts.mint.to_account_info(),
            token_program_id: ctx.accounts.token_program.to_account_info(),
        }))?;
        Ok(())
    }

    pub fn issue_credential(ctx: Context<IssueCredential>) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.recipient_ata.to_account_info(),
                authority: ctx.accounts.mint.to_account_info(),
            },
        );
        mint_to(cpi_ctx,1)?;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializeCredentialMint<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 82 + 82,
        owner = token_program.key(),
        seeds = [
            b"mint",
            
        ],
        bump
    )]
    pub mint : InterfaceAccount<'info,Mint>,
    #[account(mut)]
    pub payer : Signer<'info>,
    
    pub system_program : Program<'info,System>,
    pub token_program : Program<'info,Token>,

    
}

#[derive(Accounts)]
pub struct IssueCredential<'info> { 
    #[account(
        mut,
        seeds = [
            b"mint",
            
        ],
        bump,
        constraint = mint.mint_authority.unwrap() == authority.key() @ ErrorCodes::InvalidMintAuthority,
    )]
    pub mint:InterfaceAccount<'info,Mint>,

    #[account(mut)]
    pub authority:Signer<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = recipient,
        associated_token::token_program = token_program,
    )]
    pub recipient_ata : InterfaceAccount<'info,TokenAccount>,
    
    #[account(mut)]
    pub recipient:Signer<'info>,

    pub associated_token_program : Program<'info,AssociatedToken>,
    pub system_program : Program<'info,System>,
    pub token_program : Program<'info,Token>,

}

#[error_code]
pub enum ErrorCodes{
    InvalidMintAuthority
}



