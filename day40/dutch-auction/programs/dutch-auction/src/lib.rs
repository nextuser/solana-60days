use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke};
use anchor_spl::token::{self, Mint, Token, TokenAccount,Transfer};
use anchor_spl::associated_token::AssociatedToken;
declare_id!("3nyFiFHe2RuBJmEAZbG9cFoTYgPvk3dzAVqP65dfwKHS");

#[program]
pub mod dutch_auction {

    use super::*;

    pub fn initialize_auction(
        ctx: Context<InitializeAuction>, 
        starting_price: u64,
        floor_price: u64,
        duration : i64,
    ) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        auction.seller = *ctx.accounts.seller.key;

        auction.starting_price = starting_price;
        auction.floor_price = floor_price;
        auction.duration = duration;

        auction.starting_time = Clock::get()?.unix_timestamp;
        auction.token_mint = ctx.accounts.mint.key();
        let cpi_accounts = Transfer{
            from : ctx.accounts.seller_ata.to_account_info(),
            to : ctx.accounts.vault_ata.to_account_info(),
            authority : ctx.accounts.seller.to_account_info()
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(),
                                                                    cpi_accounts);
        token::transfer(cpi_ctx,1)?;

        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }


    pub fn buy(ctx: Context<Buy>)-> Result<()>{ 

        require!(ctx.accounts.auction.sold == false,AuctionError::NTFAlreadySold);
        let auction = &mut ctx.accounts.auction;
        let now = Clock::get()?.unix_timestamp;

        require!(now >= auction.starting_time,AuctionError::AuctionNotStarted);
        require!(now <= auction.starting_time + auction.duration,AuctionError::AuctionEnded);

        let ellapseTime = (now - auction.starting_time).min(auction.duration) as u64;
        let total_price_drop = auction.starting_price - auction.floor_price;
        let price_dropped_sofar = total_price_drop * ellapseTime/ auction.duration  as u64;
        let price = auction.starting_price - price_dropped_sofar;

        require!(ctx.accounts.buyer.lamports() >= price,AuctionError::InsufficientFunds);

        // buyer => seller: lamport

        invoke(&system_instruction::transfer(
            &ctx.accounts.buyer.key(), 
            &ctx.accounts.seller.key(), 
            price),
        
        &[ctx.accounts.buyer.to_account_info(),
            ctx.accounts.seller.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ])?;

        // seller => buyer: NFT
        let auction_key = ctx.accounts.auction.key();
        let vault_auth_bump = ctx.bumps.vault_auth;
        
        let transfer_ntf_accounts = Transfer{
            from:ctx.accounts.vault_ata.to_account_info(),
            to:ctx.accounts.buyer_ata.to_account_info(),
            authority:ctx.accounts.vault_auth.to_account_info()
        };

        let vault_signer_seed :&[&[u8]] = &[b"vault", auction_key.as_ref(), &[vault_auth_bump]];
        let signer_seeds: &[&[&[u8]]] = &[vault_signer_seed];

        let ntf_ctx =CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_ntf_accounts
            ,  signer_seeds);
        token::transfer(ntf_ctx,       1  )?;

        Ok(())

    }
}

#[derive(Accounts)]
pub struct Buy <'info>{ 
    #[account(mut,has_one = seller)]
    pub auction     : Account<'info, Auction>,
    #[account(mut  )]
    pub seller : Signer<'info>,

    pub buyer: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = auction.token_mint,
        associated_token::authority = buyer,
    )]
    pub buyer_ata: Account<'info, TokenAccount>,

    /// CHECK: vault auth
    #[account(
        seeds = [b"vault", auction.key().as_ref()],
        bump,
    )]
    pub vault_auth: AccountInfo<'info>,

    #[account(
        mut,
        associated_token::mint = auction.token_mint,
        associated_token::authority = vault_auth,
    )]
    pub vault_ata: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeAuction <'info>{
    #[account(
        init,
        payer = seller,
        space = 8 + std::mem::size_of::<Auction>(),
    )]
    pub auction: Account<'info, Auction>,

    #[account(mut)]
    pub seller : Signer<'info>,

    #[account(mut,
        associated_token::mint = mint,
        associated_token::authority = seller,
    )]
    seller_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint : Account<'info, Mint>,

    /// CHECK: vault auth
    #[account(
        seeds = [b"vault", auction.key().as_ref()],
        bump,
    )]
    pub vault_auth: UncheckedAccount<'info>,

    #[account(
        init,
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = vault_auth,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Auction {
    pub seller: Pubkey,
    pub starting_price: u64,
    pub floor_price: u64,
    pub duration: i64,
    pub starting_time: i64,
    pub token_mint: Pubkey,
    pub sold:bool,
}

#[error_code]
pub enum AuctionError {
    #[msg("Auction has not started")]
    AuctionNotStarted,

    #[msg("Buyer has insufficient funds")]
    InsufficientFunds,

    #[msg("Auction has ended")]
    AuctionEnded,
    
    #[msg("Auction has already ended")]
    AuctionAlreadyEnded,

    #[msg("NTF is already sold")]
    NTFAlreadySold,
}
