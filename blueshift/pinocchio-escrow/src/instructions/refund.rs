

use pinocchio::{
    AccountView,
    ProgramResult,
    error::ProgramError,
    cpi::{Signer,Seed},
};
use pinocchio_token::instructions::{TransferChecked,CloseAccount as CloseTokenAccount};
use pinocchio_token::state::{Mint};

use super::util::{
    close_system_account,
    token_account_init_if_needed,
    signer_check
};

use crate::instructions::{Escrow, ata_check, mint_check};

// vault_a => maker_ata_a : token_a
#[allow(dead_code)]
pub struct RefundAccounts<'a>{
    maker : &'a AccountView,
    escrow : &'a AccountView,
    mint_a : &'a AccountView,
    vault_a : &'a AccountView,
    maker_ata_a : &'a AccountView,
    system_program : &'a AccountView,
    token_program : &'a AccountView,
}


pub struct Refund<'a>{

    accounts : RefundAccounts<'a>,
}

impl <'info> Refund<'info>{
    pub const DISCRIMINATOR : &'info u8 = &2;
    pub fn process(&self) -> ProgramResult
    {
        let RefundAccounts{
            maker,
            escrow,
            mint_a,
            vault_a,
            maker_ata_a,
            system_program,
            token_program,
        } = self.accounts;

        let escrow_data= Escrow::from_account_view(escrow)?;
        if escrow_data.maker.ne( maker.address()){
            return Err(ProgramError::InvalidAccountData);
        }
        if escrow_data.mint_a.ne( mint_a.address()){
            return Err(ProgramError::InvalidAccountData);
        }


        

        let seed_binding = escrow_data.seed.to_le_bytes();
        let seeds = [
            Seed::from(crate::Escrow::SEED),
            Seed::from(escrow_data.maker.as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&escrow_data.bump),
        ];
        let signer = Signer::from(&seeds);
        let mint_info = Mint::from_account_view(mint_a)?;

        TransferChecked{
            from : vault_a,
            to : maker_ata_a,
            mint : mint_a,
            authority : escrow,
            amount : escrow_data.amount,
            decimals : mint_info.decimals(),
        }.invoke_signed(&[signer.clone()])?;

        CloseTokenAccount{
            account : vault_a,
            destination : maker,
            authority : escrow,
        }.invoke_signed(&[signer])?;
        close_system_account( escrow,maker)
    }
}

impl<'info> TryFrom<&'info [AccountView]> for Refund<'info> { 
    type Error = ProgramError;
    fn try_from(accounts: &'info [AccountView]) -> Result<Self, Self::Error> {
        let [    
                maker ,
                escrow ,
                mint_a ,
                vault_a ,
                maker_ata_a ,
                system_program ,
                token_program ,
            ] = accounts else{
                return Err(ProgramError::NotEnoughAccountKeys);
            };

            signer_check(maker)?;
            ata_check(vault_a,escrow, token_program,mint_a)?;
            mint_check(mint_a, token_program)?;

            token_account_init_if_needed(maker, maker_ata_a, maker_ata_a, token_program, mint_a, system_program)?;

            Ok(Refund {
                accounts: RefundAccounts{
                    maker,
                    escrow,
                    mint_a,
                    vault_a,
                    maker_ata_a,
                    system_program,
                    token_program
                }
            })
    }

}