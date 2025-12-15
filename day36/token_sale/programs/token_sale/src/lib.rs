use anchor_lang::prelude::*;
use anchor_lang::system_program::{
    transfer,
    Transfer,
};
use anchor_spl::token::{Mint, Token, TokenAccount,mint_to,MintTo};

declare_id!("3DiCCp2ZLQxaR6Po4osjoCZbicH1menxLsrHqF5Q82xa");

const TOKEN_PER_SOL : u64 = 100;
const SUPPLY_LIMIT : u64 = 1000 * 1000_000_000    ;

#[program]
pub mod token_sale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        
        ctx.accounts.admin_config.admin = ctx.accounts.admin.key();
        
        
        Ok(())
    }


    pub fn mint(ctx:Context<MintTokens>, lamports: u64) -> Result<()> {
        let amount = lamports.checked_mul(TOKEN_PER_SOL).ok_or(Errors::Overflow)?;
        let current_supply = ctx.accounts.mint.supply;
        let new_supply = current_supply.checked_add(amount).ok_or(Errors::Overflow)?;

        require!(new_supply <= SUPPLY_LIMIT, Errors::SupplyLimit);
        let transfer_intruction = Transfer{
            from : ctx.accounts.buyer.to_account_info(),
            to:ctx.accounts.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_intruction,
        );
        transfer(cpi_ctx, lamports)?;   

        let bump  = ctx.bumps.mint;
        let signer_seeds :&[&[ &[u8]]] = &[
            &[b"token_mint".as_ref(),&[bump]]
        ];
        let mint_to_instruction = MintTo{
            mint:ctx.accounts.mint.to_account_info(),
            to:ctx.accounts.buyer_ata.to_account_info(),
            authority:ctx.accounts.mint.to_account_info(),
        };

        let cpi_ctx: CpiContext<'_, '_, '_, '_, MintTo<'_>> = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            mint_to_instruction,
            signer_seeds,
        );
        
        mint_to(cpi_ctx, amount)?;
        
        Ok(())

    }

    pub fn withdraw(ctx:Context<WithdrawFunds>,amount:u64) -> Result<()> {
        let balance = ctx.accounts.treasury.lamports();
        require!(balance >= amount, Errors::InsufficientFunds);
        require!(ctx.accounts.admin.key() == ctx.accounts.admin_config.admin,Errors::UnauthorizedAccess);
        
        let bump = ctx.bumps.treasury;
        let seeds :&[&[ &[u8]]] = &[
            &[b"treasury".as_ref(),
            &[bump]]
        ];
        // let cpi_ctx = CpiContext::new_with_signer(
        //     ctx.accounts.system_program.to_account_info(),
        //     Transfer{
        //         from:ctx.accounts.treasury.to_account_info(),
        //         to:ctx.accounts.admin.to_account_info(),
        //     },
        //     seeds,
        // );

        // transfer(cpi_ctx, amount)?;

        // 直接从treasury转移lamports到admin，而不是使用系统程序的CPI调用
        **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.admin.to_account_info().try_borrow_mut_lamports()? += amount;
        
        msg!("Withdrew {} lamports from treasury to admin", amount);
        Ok(() )
    }

}

#[derive(Accounts)]
pub struct WithdrawFunds<'info>{
    #[account(mut)]
    pub admin :Signer<'info>,
    /// CHECK:
    pub admin_config : Account<'info, AdminConfig>,
   
    #[account(mut , seeds=[b"treasury"],bump)]
    /// CHECK: treausry is pda account
    pub treasury : AccountInfo<'info>,
    system_program:Program<'info, System>,

}



#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(mut)]
    pub admin :Signer<'info>,

    #[account(
    init,
    payer= admin,
    space = std::mem::size_of::<AdminConfig>() + 8,
    )]
    pub admin_config : Account<'info, AdminConfig>,

    /// CHECK:
    #[account(
        init,
        payer= admin,
        seeds=[b"token_mint"],
        bump,
        mint::decimals = 9,
        //在初始化过程中，我们设置了 ` mint::authority = mint.key()`，使铸造 PDA 拥有了独立的权限。
        mint::authority = mint.key(),
    )]
    pub mint:Account<'info,Mint>,
    
    /// CHECK:  这里和rareskill的不同，需要initialize treasury account,保证owner为本程序，避免账户被其他用后窃取sol
    #[account(
        init,
        payer= admin,
        space = 0,
        //mut,
        seeds=[b"treasury"],
        bump,
    )]
    pub treasury:AccountInfo<'info>,
    pub token_program: Program<'info, Token>, 
    pub system_program:Program<'info, System>,
}

#[account]
pub struct AdminConfig{
    pub admin : Pubkey,
}



#[derive(Accounts)]
pub struct   MintTokens<'info>{
    #[account(mut)]
    pub buyer : Signer<'info>,
    /// CHECK:
    pub admin:AccountInfo<'info>,
       
    #[account(
        mut,
        seeds=[b"token_mint"],
        bump)]
    pub mint : Account<'info, Mint>,


    #[account(
        mut,
        token::mint=mint,
        token::authority=buyer)]
    pub buyer_ata : Account<'info, TokenAccount>,
    
    /// CHECK:   
    #[account(
        mut,
        seeds=[b"treasury"],
        bump,
    )]
    pub treasury : AccountInfo<'info>,
    

    pub token_program : Program<'info, Token>,
    pub system_program : Program<'info, System>,
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



