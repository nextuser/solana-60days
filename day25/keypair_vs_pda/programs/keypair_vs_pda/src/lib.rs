use anchor_lang::prelude::*;

declare_id!("3aVHf3DqteCvXJT4VSGzMBYbVYeS36fzjEYfQhsNG1R5");

#[program]
pub mod keypair_vs_pda {
    use super::*;
    pub fn initialize(ctx: Context<InitializeKeypairAccount>,value :u64) -> Result<()> {
        ctx.accounts.my_keypair_account.x = value;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[account]
pub struct KeyPairAccount{
    x : u64,
}

#[derive(Accounts)]
pub struct InitializeKeypairAccount <'info>{
    #[account(init , payer=signer, space= size_of::<KeyPairAccount>() + 8 )]
    pub my_keypair_account : Account<'info,KeyPairAccount>,
    #[account(mut)]
    signer:Signer<'info>,
    pub system_program : Program<'info,System>
}
