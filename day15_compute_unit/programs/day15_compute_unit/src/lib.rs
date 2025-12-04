use anchor_lang::prelude::*;

declare_id!("DvbJSxS1iwx9mauTuFiiy2FTJbp8Yncpr36k88Frstwh");

// #[program]
// pub mod day15_compute_unit {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         msg!("Greetings from: {:?}", ctx.program_id);
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}




#[derive(Accounts)]
pub struct Empty{
}

#[derive(Accounts)]
pub struct Signed <'info>{
    signer : Signer<'info>,
    secondSigner : Signer<'info>,
}

#[program]
pub mod compute_unit{
    use super::*;
    pub fn initialize(ctx : Context<Empty>) -> Result<()>{
        
        Ok(())
    }

    pub fn call_signed(ctx : Context<Signed>) -> Result<()>{
        
        Ok(())
    }
}