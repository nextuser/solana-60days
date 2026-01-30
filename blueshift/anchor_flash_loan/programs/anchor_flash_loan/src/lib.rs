use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, TokenAccount,Mint,transfer,Transfer},
    associated_token::AssociatedToken,
};
use anchor_lang::{
    Discriminator,
    solana_program::sysvar::instructions::{
        ID  as INSTRUCTIONS_SYSVAR_ID,
        load_instruction_at_checked,
    }
};

declare_id!("2zdW2PReARyVmUrKk7dKSmoumf2fkTPAcawEUuU4KHPA");

#[program]
pub mod anchor_flash_loan {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
