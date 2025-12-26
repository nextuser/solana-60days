use anchor_lang::prelude::*;
// use anchor_lang::solana_program::{
//     ed25519_program,
//     pubkey::Pubkey,
//     sysvar::instructions as sysvar_instructions,
//     sysvar::SysvarId,
// };

use anchor_lang::solana_program::{
    //ed25519_program,
    pubkey::Pubkey,
    sysvar::instructions as sysvar_instructions,
};
const ED25519_PROGRAM_ID : Pubkey= pubkey::pubkey!("Ed25519SigVerify111111111111111111111111111");


declare_id!("D7Hzu8LJLUQzCaqPbnQdhKtsSa481Qo5QBTnaK3SjELK");

#[program]
pub mod airdrop {
    use super::*;

    const HEADER_LEN :usize = 16;
    const PUBKEY_LEN :usize = 32;
    const SIGNATURE_LEN :usize = 64;
    const MSG_LEN :usize = 40; // recipient(32) + amoutn(8)
    const SHIFT :usize = 2;
    const THIS_INDEX : usize = u16::MAX as usize;

    pub fn claim(ctx: Context<Claim >) -> Result<()> {
        let ix_sysvar_count = ctx.accounts.instructionsysvar.to_account_info();
        let current_instraction_index = sysvar_instructions::load_current_index_checked(&ix_sysvar_count)
            .map_err(|_| error!(AirdropError::InvalidInstructionSysvar) )? ;
        require!(current_instraction_index > 0,AirdropError::NoSignatureFound);

        //加载签名一条指令
        let prev_index = current_instraction_index.checked_sub(1).unwrap() as usize;
        let ed25519_instruction = sysvar_instructions::load_instruction_at_checked(prev_index, &ix_sysvar_count)
            .map_err(|_| error!(AirdropError::InvalidInstructionSysvar) )?;

        require!(ed25519_instruction.program_id == ED25519_PROGRAM_ID,AirdropError::InvalidInstructionSysvar);

        require!(ed25519_instruction.accounts.is_empty(),AirdropError::BadEd25519Account);    
        require!(ed25519_instruction.data.len() > HEADER_LEN,AirdropError::BadEd25519HEADER_LENGTH);
        let sig_count = ed25519_instruction.data[0] as usize;
        require!(sig_count > 0,AirdropError::BadEd25519HEADER_LENGTH);
        let shift_data = &ed25519_instruction.data[SHIFT..];
        
        let signature_offset = read_u16(shift_data, 0)? as usize;
        let signature_instruction_index = read_u16(shift_data, 1)? as usize;
        let publickey_offset = read_u16(shift_data, 2)? as usize;
        let publickey_instruction_index = read_u16(shift_data, 3)? as usize;
        let message_offset = read_u16(shift_data, 4)? as usize;
        let message_length = read_u16(shift_data, 5)? as usize;
        let message_instruction_index = read_u16(shift_data, 6)? as usize;
        let data_len = ed25519_instruction.data.len();

        let data = & ed25519_instruction.data[..];

        

        require!(signature_instruction_index == THIS_INDEX, AirdropError::InvalidInstructionSysvar);
        require!(publickey_instruction_index == THIS_INDEX, AirdropError::InvalidInstructionSysvar);
        require!(message_instruction_index == THIS_INDEX, AirdropError::InvalidInstructionSysvar);

        require!(signature_offset>= HEADER_LEN, AirdropError::InvalidInstructionSysvar);
        require!(publickey_offset>= HEADER_LEN, AirdropError::InvalidInstructionSysvar);
        require!(message_offset>= HEADER_LEN, AirdropError::InvalidInstructionSysvar);
        require!(signature_offset + SIGNATURE_LEN<= data_len, AirdropError::InvalidInstructionSysvar);
        require!(publickey_offset + PUBKEY_LEN<= data_len, AirdropError::InvalidInstructionSysvar);
        require!(message_offset + message_length <= data_len, AirdropError::InvalidInstructionSysvar);
        let pk_slice = &data[publickey_offset..publickey_offset+PUBKEY_LEN];
        let mut pk_arr = [0u8;32];
        pk_arr.copy_from_slice(pk_slice);


        let mut amount_bytes = [0u8;8];


        require!(message_length == MSG_LEN , AirdropError::InvalidInstructionSysvar);
        let msg =  &data[message_offset..message_offset + message_length];
        amount_bytes.copy_from_slice(&msg[32..40]);
        // airdrop token to  recipient
        let amount = u64::from_le_bytes(amount_bytes);        
        let distributor_pk = Pubkey::new_from_array(pk_arr);

        msg!("message amount {}, publickey {}", amount, distributor_pk.to_string());
        if distributor_pk != ctx.accounts.expected_distributor.key(){
            msg!("distributor mismatch, {}, expected={}",
                distributor_pk, ctx.accounts.expected_distributor.key());
                //return Ok(());
            return Err(error!(AirdropError::DistributorMismatch));
        }

        let mut rec_arr = [0u8;32];
        rec_arr.copy_from_slice(&msg[0..32]);
        let recipient_pk = Pubkey::new_from_array(rec_arr);
        if recipient_pk != ctx.accounts.recipient.key() {
            return Err(error!(AirdropError::RecipientMismatch));
        }


        
        do_airdrop(&distributor_pk, &recipient_pk, amount);
        Ok(())
    }
}

pub fn do_airdrop(distributor: &Pubkey, recipient: &Pubkey, amount: u64){
    msg!("Airdrop {} tokens from {} to {}", amount, distributor, recipient);
}

#[derive(Accounts)]
pub struct Claim <'info> {
    #[account(mut)]
    pub recipient: Signer<'info>,
    /// CHECK: expected distributor public key
    pub expected_distributor: UncheckedAccount<'info>,
    
    /// CHECK: sysvar instructions account
    #[account(address = sysvar_instructions::id())]
    pub instructionsysvar : AccountInfo<'info>,

    pub system_program : Program<'info, System>,
}

pub fn read_u16(data: &[u8], i: usize) -> Result<u16>
{

    let start =  i.checked_mul(2).ok_or(error!(AirdropError::Overflow))?;
    let end = start.checked_add(2).ok_or(error!(AirdropError::Overflow))?;
    let src = data.get(start..end).ok_or(error!(AirdropError::Overflow))?;
    let mut arr = [0u8; 2];
    arr.copy_from_slice(src);
    Ok(u16::from_le_bytes(arr))
        
}

#[error_code]
pub enum AirdropError {
    #[msg("Invalid instruction sysvar")]
    InvalidInstructionSysvar,
    #[msg("Bad Ed25519 program")]
    BadEd25519Program,
    #[msg("Bad Ed25519 account")]
    BadEd25519Account,
    #[msg("Distributor mismatch")]
    DistributorMismatch,
    #[msg("Recipient mismatch")]
    RecipientMismatch,
    #[msg("Bad Ed25519 header length")]
    BadEd25519HEADER_LENGTH,
    #[msg("index overflow")]
    Overflow,



    NoSignatureFound,

}
