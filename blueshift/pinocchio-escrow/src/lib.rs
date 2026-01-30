
use pinocchio::{
    AccountView, ProgramResult, address::Address, 
    default_allocator, default_panic_handler, program_entrypoint,
    error::ProgramError, 
};

mod instructions;
use instructions::*;

const ID : Address = Address::from_str_const("22222222222222222222222222222222222222222222");
type ArgType<'info> = (&'info [u8],&'info [AccountView]);


program_entrypoint!(process_instructions);
// default_allocator!();
// default_panic_handler!();
pub fn process_instructions(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8]
) -> ProgramResult 
{
    // if program_id.ne( &ID) {
    //     return Err(ProgramError::IncorrectProgramId);
    // }
    let parsed  = instruction_data.split_first();
    match parsed {
        Some((Make::DISCRIMINATOR, data)) => Make::try_from(( data, accounts))?.process(),
        Some((Take::DISCRIMINATOR, _)) => Take::try_from( accounts)?.process(),
        Some((Refund::DISCRIMINATOR, _)) => Refund::try_from( accounts)?.process(),   
        _ => Err(ProgramError::InvalidInstructionData),
    }

}