use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer,Transfer};
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("3DiCCp2ZLQxaR6Po4osjoCZbicH1menxLsrHqF5Q82xa");

const TOKEN_PER_SOL : u64 = 100;
const SUPPLY_CAP : u64 = 1000 * 1000_000_000    ;

#[program]
pub mod token_sale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        
        ctx.accounts.admin_config.admin = ctx.accounts.admin.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(mut)]
    pub admin :Signer<'info>,

    #[account(
    init,
    payer= admin,
    space = AdminConfig::INIT_SPACE + 9,
    )]
    pub admin_config : Account<'info, AdminConfig>,

    /// CHECK:
    #[account(
        init,
        payer= admin,
        seeds=[b"token_mint"],
        bump,
        mint::decimals = 9,
        mint::authority = admin.key(),
    )]
    pub mint:Account<'info,Mint>,
    
    /// CHECK:
    #[account(
        seeds=[b"treasury"],
        bump
    )]
    pub treasury:AccountInfo<'info>,
    pub token_program: Program<'info, Token>, 
    pub system_program:Program<'info, System>,

}

#[account]
#[derive(InitSpace)]
pub struct AdminConfig{
    pub admin : Pubkey,
}

#[error_code]
pub enum Errors{
    #[msg("Max token supply limit reached")]
    SupplyLimit,
    #[msg("Math overflow")]
    Overflow,
    #[msg("Only admin can withdraw")]
    UnauthorizedAccess,
    #[msg("Not enough Sol in treasury")]
    InsufficientFunds,
}


#[test]
fn test_space(){
    msg!("init space {},sizeof {}", AdminConfig::INIT_SPACE, std::mem::size_of::<AdminConfig>());
    assert_eq!(AdminConfig::INIT_SPACE, std::mem::size_of::<AdminConfig>());
}
