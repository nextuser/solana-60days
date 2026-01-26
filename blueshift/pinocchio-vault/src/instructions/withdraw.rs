use pinocchio::account_info::AccountInfo;
use pinocchio::ProgramResult;
use pinocchio::program_error::ProgramError;
use pinocchio::instruction::Signer;
use pinocchio::instruction::Seed;
use pinocchio_system::instructions::Transfer;
use crate::util::derive_vault_address;
pub struct Withdraw <'info>{
    pub authority : &'info AccountInfo,
    pub vault: &'info AccountInfo,
    
}

impl <'info> Withdraw <'info>{
     pub const DISCRIMINATOR: &'info u8 = & 1u8;

    pub fn process(&self) -> ProgramResult {
        let (expected_pda,bump) = derive_vault_address(self.authority.key());
        if self.vault.key().ne(&expected_pda){
            return Err(ProgramError::InvalidAccountData)
        }
        let bump_arr = [bump];
        let seeds = [
                Seed::from(b"vault"),
                Seed::from(self.authority.key().as_ref()),
                Seed::from(&bump_arr),
            ];
        let signer = Signer::from(
            &seeds
        );

        Transfer{
            from:self.vault,
            to:self.authority,
            lamports:self.vault.lamports(),
        }.invoke_signed(&[signer])?;

        Ok(())
    }

}

impl <'info> TryFrom< &'info [AccountInfo] > for Withdraw <'info> {
    type Error = ProgramError;

    fn try_from(accounts : &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let authority = accounts.get(0).ok_or(ProgramError::InvalidAccountData)?;
        let vault = accounts.get(1).ok_or(ProgramError::InvalidAccountData)?;
        if !vault.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        if vault.lamports().eq(&0) {
            return Err(ProgramError::InvalidAccountData);
        }

        let withdraw = Withdraw {
            authority,
            vault,
        };
        Ok(withdraw)
    }
}