use anchor_lang::prelude::sysvar::instructions::{load_current_index_checked, load_instruction_at_checked};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    system_program,  
    sysvar::instructions,
    system_instruction::SystemInstruction,
};
use bincode;
declare_id!("BJbmrpLQZu2MqDjdVzmT5Hw9Bi1iFU8BtdVxg5b379Be");

#[program]
pub mod check_transfer {
    use super::*;

    pub fn verify_transfer(ctx: Context<VerifyTransfer>,expected_lamports:u64) -> Result<()> {
    
        // step 1
        let  current_ix_indx = instructions::load_current_index_checked(&ctx.accounts.instruction_sysvar)?;
        if current_ix_indx == 0 {
            return Err(error!(ErrorCodes::MissingInstruction));
        }
        let prev_index = current_ix_indx.checked_sub(1).unwrap() as usize;

        let transfer_ix = load_instruction_at_checked(
                                       prev_index,
                                        &ctx.accounts.instruction_sysvar)
                                        .map_err(|_| error!(ErrorCodes::MissingInstruction))?;
        // step 2
        require_keys_eq!(transfer_ix.program_id, system_program::ID, ErrorCodes::NotSystemTransfer);
        let system_ix = bincode::deserialize(&transfer_ix.data)
            .map_err(|_| error!(ErrorCodes::InvalidInstructionData))?;
        match system_ix {
            SystemInstruction::Transfer { lamports } => { 
                require!(lamports == expected_lamports, ErrorCodes::IncorrectAmount);
            },
            _ => return Err(error!(ErrorCodes::NotSystemTransfer)),
        }

        require_gte!(transfer_ix.accounts.len(), 2, ErrorCodes::InsufficientAccoutns);
        let from_account = &transfer_ix.accounts[0];
        let to_account = &transfer_ix.accounts[1];
        require!(from_account.is_signer, ErrorCodes::NotSigner);
        require!(from_account.is_writable, ErrorCodes::FromAccountNotWritable);
        require!(to_account.is_writable, ErrorCodes::ToAccountNotWritable);

        msg!("transfer from {} to {}",from_account.pubkey, to_account.pubkey);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VerifyTransfer <'info> {

  /// CHECK:
    #[account(address =  instructions::ID)]
    pub instruction_sysvar:AccountInfo<'info>,   
}

#[error_code]
pub enum ErrorCodes {
    #[msg("Missing required instruction in transaction")]
    MissingInstruction,
    #[msg("Instruction is not a from system  program")]    
    NotSystemTransfer,
    #[msg("Invalid instruction data format")]
    InvalidInstructionData,
    #[msg("Incorrect amount")]
    IncorrectAmount,
    #[msg("Insufficient accounts")]
    InsufficientAccoutns,
    #[msg("From account is not a signer")]
    NotSigner,
    #[msg("From account is not writable")]
    FromAccountNotWritable,
    #[msg("To account is not writable")]
    ToAccountNotWritable,
}