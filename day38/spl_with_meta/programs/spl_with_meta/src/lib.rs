use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use mpl_token_metadata::instructions::{
    CreateMetadataAccountV3,CreateMetadataAccountV3CpiAccounts,
    CreateMetadataAccountV3InstructionArgs,CreateMetadataAccountV3Cpi,
};

use mpl_token_metadata::types::{Creator,DataV2};
use mpl_token_metadata::ID as METADATA_PROGRAM_ID;



declare_id!("AnLGBowxWXerdTqWbE9cMtadDsHfou6gNyKuUuTZaSeN");

#[program]
pub mod spl_with_meta {

    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }

    pub fn create_token_metadata( 
        ctx:Context<CreateTokenMetadata>,
        name: String,
        symbol: String,
        uri: String,
        seller_fee_basis_points: u16,
        is_mutable: bool,
    ) -> Result<()> { 
        let data = DataV2{
            name,
            symbol,
            uri,
            seller_fee_basis_points,
            creators: Some(vec![
                Creator{
                    address: ctx.accounts.payer.key(),
                    verified: true,
                    share: 100,
                }
            ]),
            collection: None,
            uses: None,
        };
        let mint_key = ctx.accounts.mint.key();
        let seeds = &[b"metadata", METADATA_PROGRAM_ID.as_ref(), &mint_key.to_bytes()];
        let (pda, _bump) = Pubkey::find_program_address(seeds, &METADATA_PROGRAM_ID);
        require!(pda == ctx.accounts.metadata.key(), MetaplexError::InvalidMetadataAccount);
        let token_metadata_program_info = ctx.accounts.token_metadata_program.to_account_info();
        let metadata_info = ctx.accounts.metadata.to_account_info();
        let mint_info = ctx.accounts.mint.to_account_info();
        let payer_info= ctx.accounts.payer.to_account_info();
        let system_program_info = ctx.accounts.system_program.to_account_info();
        let rent_info = ctx.accounts.rent.to_account_info();
        let authority_info = ctx.accounts.authority.to_account_info();

        let cpi = CreateMetadataAccountV3Cpi::new(
            &token_metadata_program_info,
            CreateMetadataAccountV3CpiAccounts{
                metadata: &metadata_info,
                mint: &mint_info,
                update_authority: (&authority_info,true),
                mint_authority: &authority_info,
                payer: &payer_info,
                system_program: &system_program_info,
                rent: Some(&rent_info),
            },
            CreateMetadataAccountV3InstructionArgs { data, is_mutable, collection_details: None }
        );

        cpi.invoke()?;       

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTokenMetadata <'info>{
    ///  CHECK: This account is used for METADATA PDA
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub authority:Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

    /// CHECK: This account is used for METADATA_PROGRAM
    #[account(address = METADATA_PROGRAM_ID)]
    pub token_metadata_program: AccountInfo<'info>,


}

#[error_code]
pub enum MetaplexError {
    #[msg("Invalid Metadata Account")]
    InvalidMetadataAccount,
}
