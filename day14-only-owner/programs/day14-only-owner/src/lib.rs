use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use std::str::FromStr;
declare_id!("CGMd57opvCJB7gXoN6E3goDDT3W7LcT4s2BEJLbEAJKN");
const OWNER :&str = "JYLqLj2ztNMHUKXjqPthg9UiYWzkkBgGh6xJvg2v4hp";
#[program]
pub mod day14_only_owner {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("the payer from: {:?}",  ctx.accounts.payer.key.to_string());
        msg!("the signer from : {:?}",ctx.accounts.signer.key.to_string());
        Ok(())
    }

    pub fn call(ctx:Context<OnlyOwner>) ->Result<()>{
        //check if owner == signer;
        let ownerKey :Pubkey = Pubkey::from_str(OWNER).unwrap();
        require_keys_eq!(ctx.accounts.payer.key(), ownerKey, OnlyOwnerError::NotOwner);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    pub payer : Signer<'info>,
    pub signer : Signer<'info>
}

#[derive(Accounts)]
pub struct OnlyOwner<'info>{
    payer : Signer<'info>
}

#[error_code]
pub enum OnlyOwnerError{
    #[msg("Only Owner can call this function")]
    NotOwner,
}
