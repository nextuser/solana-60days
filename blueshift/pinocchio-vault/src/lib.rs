//#![no_std]

use pinocchio::{
    ProgramResult, account_info::AccountInfo, 
    program_entrypoint, default_panic_handler,default_allocator,
    program_error::ProgramError, pubkey::Pubkey, 
};
mod instructions;
mod util;
use instructions::*;

use pinocchio_pubkey::{declare_id};
program_entrypoint!(process_instruction);
default_allocator!();
default_panic_handler!();




declare_id!("22222222222222222222222222222222222222222222");
type ArgType<'a> = ( &'a [u8],&'a [AccountInfo]);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Deposit::DISCRIMINATOR,data)) => {
            let deposit = Deposit::try_from((data,accounts)).map_err(|_| ProgramError::InvalidInstructionData)? ;
            deposit.process()?;  

            Ok(())
        },
        Some((Withdraw::DISCRIMINATOR,_data)) => {
            let withdraw = Withdraw::try_from(accounts).map_err(|_| ProgramError::InvalidInstructionData)? ;
            withdraw.process()?;  

            Ok(())
        },  
        _ => return Err(ProgramError::InvalidInstructionData),
    }
}