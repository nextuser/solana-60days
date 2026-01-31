

use pinocchio::{
    AccountView,
    ProgramResult,
    error::ProgramError,
    cpi::{Signer,Seed},
};
use pinocchio_token::instructions::{Transfer as TokenTransfer,CloseAccount as CloseTokenAccount};
use pinocchio_token::state::{Mint,TokenAccount};
use solana_program_log::log;
use super::util::{
    close_system_account,
    token_account_init_if_needed,
    signer_check
};
use crate::EscrowError;
use crate::instructions::{Escrow, ata_check};

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
        #[allow(unused_variables)]
        let RefundAccounts{
            maker ,
            escrow ,
            mint_a ,
            vault_a ,
            maker_ata_a ,
            system_program ,    
            token_program 
            
        } = self.accounts;

        // let escrow_data= Escrow::from_account_view(escrow)?;
        // if escrow_data.maker.ne( maker.address()){
        //     return Err(ProgramError::InvalidAccountData);
        // }
        // log!("process 3");

        let (seed,bump) ={
            let escrow_data  = Escrow::from_account_view(escrow)?;
            if escrow_data.maker.ne(maker.address()) {
                return Err(EscrowError::EscrowMakerMismatch.into());
            }

            (escrow_data.seed,escrow_data.bump)
        };
        

        let seed_binding = seed.to_le_bytes();
        let bump_binding = [bump];
        let seeds = [
            Seed::from(crate::Escrow::SEED_PREFIX),
            Seed::from(maker.address().as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&bump_binding),
        ];
        let signer = Signer::from(&seeds);
        //let mint_info = Mint::from_account_view(mint_a)?;
        let vault_amount = TokenAccount::from_account_view(vault_a)?.amount();
        TokenTransfer{
            from : vault_a,
            to : maker_ata_a,
            authority : escrow,
            amount : vault_amount,
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
                _
                            
            ] = accounts else{
                return Err(ProgramError::NotEnoughAccountKeys);
            };

            signer_check(maker)?;

            // log!("ata check");

            ata_check(vault_a,escrow, token_program,mint_a.address())?;

            token_account_init_if_needed(maker, maker_ata_a, maker, token_program, mint_a, system_program)?;

            Ok(Refund {
                accounts: RefundAccounts{
                    maker,
                    mint_a,
                    escrow,
                    vault_a,
                    maker_ata_a,
                    system_program,
                    token_program
                }
            })
    }

}