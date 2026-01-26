use pinocchio::{
    account_info::AccountInfo, 
    program_error::ProgramError,
    ProgramResult,
};

use pinocchio_system::{
     instructions::Transfer
};
use crate::util::derive_vault_address;
use crate::ArgType;

pub struct Deposit <'info>{
    pub payer : &'info AccountInfo,
    pub vault : &'info AccountInfo,
    pub amount: u64,
}

impl <'info> Deposit <'info> {
    pub const DISCRIMINATOR: &'info u8 = & 0u8;

    pub fn process(&self) -> ProgramResult {
        let (pda,_bump) = derive_vault_address(self.payer.key());

        // if ! self.payer.is_signer() {
        //     return Err(ProgramError::MissingRequiredSignature);
        // }

        if self.vault.key().ne(&pda) {
            return Err(ProgramError::InvalidAccountData);
        }
        // if self.vault.lamports() == 0 {
        //     create_account_with_minimum_balance_signed(
        //         self.vault,
        //         0,
        //         &crate::ID,
        //         self.payer,
        //         None,
        //         &[signer]
        //     )?;
        // }

        Transfer{
            from:self.payer,
            to:self.vault,
            lamports:self.amount,
        }.invoke()?;

        Ok(())


        
    }
}

impl <'info> TryFrom< ArgType<'info> > for Deposit <'info> {
    type Error = ProgramError;
    fn try_from(arg: ArgType<'info>) -> Result<Self, Self::Error> {
        let (instruction_data, accounts) = arg;
        let amount_data :[u8;8] = instruction_data.try_into().map_err(|_| ProgramError::InvalidInstructionData)?;
        let amount = u64::from_le_bytes(amount_data);
        let payer = accounts.get(0).ok_or(ProgramError::InvalidInstructionData)?;
        let vault = accounts.get(1).ok_or(ProgramError::InvalidInstructionData)?;
        if amount.eq(&0) {
                return Err(ProgramError::InvalidInstructionData);

        }
        
        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        
        if vault.lamports().ne(&0) {
            return Err(ProgramError::InvalidAccountData);
        }

        if !vault.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        
        Ok(Self {
            payer,
            vault,
            amount,
        })
    }
}


