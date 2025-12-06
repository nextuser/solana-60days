use anchor_lang::prelude::*;

declare_id!("Bj5syj7sDknn3EEVBgqEVeBDsDu8821Ft56eG6yDYv3T");

#[program]
pub mod owner {
    use super::*;

    pub fn initialize_keypair(ctx: Context<InitializeKeyPair>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn initialize_pda(ctx: Context<InitializePda>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeKeyPair<'info> {
    #[account(init,payer=signer,space=size_of::<KeypairData>() + 8 )]
    data :Account<'info,KeypairData>,
    #[account(mut)]
    signer:Signer<'info>,
    system_program:Program<'info ,System>
}

#[derive(Accounts)]
pub struct InitializePda <'info>{
    #[account(init , payer=signer,space=size_of::<Pda>() + 8 , seeds = [] , bump)]
    data :Account<'info,Pda>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program:Program<'info,System>,
}

#[account]
pub struct Pda{

}

#[account]
pub struct KeypairData{

}