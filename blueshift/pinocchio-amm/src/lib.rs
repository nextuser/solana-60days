use pinocchio::{
    address::Address,
    error::ProgramError,
    entrypoint,
    AccountView,
};

mod state;
use state::*;
mod instructions;
use instructions::*;
mod helper;
use helper::*;
mod errors;
use errors::*;

pub type ArgType<'info > = (&'info [u8], &'info [AccountView]);

// 22222222222222222222222222222222222222222222
pub const ID: Address = Address::from_str_const("22222222222222222222222222222222222222222222");


entrypoint!(process_instruction);

fn process_instruction(
    _program_id: & Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
   match instruction_data.split_first(){
        Some((Initialize::DISCRIMINATOR,data)) => Initialize::try_from((data,accounts))?.process()?,
        Some((Deposit::DISCRIMINATOR,data)) => Deposit::try_from((data,accounts))?.process()?,
        Some((Withdraw::DISCRIMINATOR,data)) => Withdraw::try_from((data,accounts))?.process()?,
        Some((Swap::DISCRIMINATOR,data)) => Swap::try_from((data,accounts))?.process()?,
        _ => Err(ProgramError::InvalidInstructionData)?,
    };

    Ok(())

}