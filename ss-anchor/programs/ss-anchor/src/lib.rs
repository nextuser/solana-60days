use anchor_lang::prelude::*;

declare_id!("5EVm3VwBxQ7cm6fQXg81WMfkL3W7xFRD24uHrw61tTjs");

#[program]
pub mod ss_anchor {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>,new_data : Vec<u8>) -> Result<()> {
        let pda = &mut ctx.accounts.user_pda;
        pda.auth = *ctx.accounts.user.key;
        pda.bump = ctx.bumps.user_pda;
        pda.data.resize(new_data.len(), 0);
        pda.data.copy_from_slice(&new_data);
        msg!("initialize pda from: {:?}", pda.key().to_string());
        Ok(())
    }

    pub fn update(ctx:Context<Update>,data : Vec<u8>) -> Result<()> {
        let user_pda = &mut ctx.accounts.user_pda;
        require!(user_pda.to_account_info().owner == ctx.program_id, StorageError::InvalidAccountOwner);
        require!(user_pda.auth == *ctx.accounts.user.key, StorageError::InvalidAuthority);

        let user = &mut ctx.accounts.user;
        let old_len = Data::space_for(user_pda.data.len());
        let new_len = Data::space_for(data.len());
        msg!("old data len: {}, new data len: {}", old_len, new_len);
        let need_lamports =  Rent::get()?.minimum_balance(new_len);
        let curr_lamports = user_pda.get_lamports();
        msg!("need lamports: {}, current lamports: {}, user balance:{}", 
                need_lamports, curr_lamports, user.lamports());
        
        if need_lamports > curr_lamports  {
            let need_fee = need_lamports.checked_sub(curr_lamports).ok_or(StorageError::Overflow)?;
            
            let system_program = &ctx.accounts.system_program;
            msg!("transfer:{}", need_fee);
            let cpi_context = CpiContext::new(
                system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: user.to_account_info(),
                    to: user_pda.to_account_info(),
                },
            );
            anchor_lang::system_program::transfer(cpi_context, need_fee)?;
            user_pda.data = data;

        } else if need_lamports < curr_lamports {
            let more_fee = curr_lamports.checked_sub(need_lamports).ok_or(StorageError::Overflow)?;

            user_pda.data = data;
            user_pda.sub_lamports(more_fee)?;
            user.add_lamports(more_fee)?;
            msg!("refund:{},update lamports:{}", more_fee, user_pda.get_lamports());
        };
        
        
        //user_pda.data = data;
        Ok(())
    }
}

const SEED : &[u8] = b"storage";
#[derive(Accounts)]
#[instruction(new_data : Vec<u8>)]
pub struct Initialize <'info>{

    #[account(mut)]
    pub user : Signer<'info>,
    #[account(
        init, 
        payer = user, 
        seeds = [SEED, user.key().as_ref()],
        bump,        
        space = Data::space_for(new_data.len()),
    )]
    pub user_pda : Account<'info, Data>,
    pub system_program : Program<'info, System>,

}

#[error_code]
pub enum StorageError{
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Overflow error")]
    Overflow,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
}



#[account]
pub struct Data {
    pub auth : Pubkey,
    pub bump: u8,
    pub data : Vec<u8>,
}

#[derive(Accounts)]
#[instruction(data : Vec<u8>)]
pub struct Update <'info>{

    #[account(mut)]
    pub user : Signer<'info>,
    #[account(
        mut,
        seeds = [SEED, user.key().as_ref()],
        bump = user_pda.bump,
        realloc = Data::space_for(data.len()),
        realloc::payer = user,
        realloc::zero = false,
        constraint = user_pda.auth == *user.key,
        
    )]
    pub user_pda : Account<'info, Data>,
    pub system_program : Program<'info, System>,

}


const DISCRIMINATOR_LENGTH: usize = 8;
const VECTOR_HEADER_LENGTH: usize = 4;

impl Data {
    pub fn space_for(data_len : usize) -> usize {
        data_len +  
        std::mem::size_of::<Pubkey>() + 
        std::mem::size_of::<u8>() + 
        DISCRIMINATOR_LENGTH + 
         VECTOR_HEADER_LENGTH
    }//END fn
}
