
use pinocchio::{
    
    AccountView,
    error::ProgramError,
    ProgramResult,
    Address
};
use pinocchio_token::state::{TokenAccount,Mint};
use crate::errors::EscrowError;
use pinocchio_associated_token_account::{
    instructions::{Create as TokenCreate},
};
use pinocchio_associated_token_account::ID as ASSOCIATED_TOKEN_PROGRAM_ID  ;
use pinocchio_token::ID as TOKEN_PROGRAM_ID;
use pinocchio_token_2022::{
    ID as TOKEN_2022_PROGRAM_ID,
    state::TokenAccount as TokenAccount2022,
};
//use solana_program_log::log;


//const TOKEN_PROGRAM_ID : Address = Address::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
//const TOKEN_2022_PROGRAM_ID : Address = Address::from_str_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
//const SYSTEM_PROGRAM_ID : Address = Address::from_str_const("11111111111111111111111111111111");
//const ASSOCIATED_TOKEN_PROGRAM_ID : Address = Address::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
//const NATIVE_MINT : Address = Address::from_str("So11111111111111111111111111111111111111112");
//const NATIVE_MINT_2022 : Address = Address::from_str("9pan9bMn5HatX4EJdBwg9VgCa7Uz5HL8N1m5D3NdXejP");S
pub fn get_ata_address(
    authority : & Address,
    mint : & Address, 
    
    token_program : & Address
    ) -> Address
{
    let seeds = [
        authority.as_ref(),
        token_program.as_ref(),
        mint.as_ref(),

    ];
    let pda = Address::find_program_address(&seeds, &ASSOCIATED_TOKEN_PROGRAM_ID).0;
    pda
}

pub fn program_check(pda : & AccountView, space : usize) -> ProgramResult
{
    if  !pda.owned_by(&crate::ID) {
        return Err(EscrowError::PdaOwnerMismatch.into());
    }

    if pda.data_len() != space{
        return Err(EscrowError::PdaSpaceMismatch.into());
    }
    Ok(())
}
pub fn mint_check(
    mint : & AccountView,
    token_program : & AccountView,
    ) -> ProgramResult
{
    if !mint.owned_by(token_program.address()){
        return Err(EscrowError::MintOwnerMismatch.into());
    }
    let token_address = token_program.address();
    
    match token_address {
        &TOKEN_PROGRAM_ID => {
            
            if mint.data_len() != Mint::LEN{
                return Err(EscrowError::MintSpaceMismatch.into());
            }
        },
        &TOKEN_2022_PROGRAM_ID => {
            if mint.data_len() < TokenAccount2022::BASE_LEN{
                return Err(EscrowError::MintSpaceMismatch.into());
            }
            
        },
        _ => {
            return Err(EscrowError::InvalidTokenProgram.into());
        }
    }
    Ok(())
}

pub fn ata_address_check(
    ata_account : & AccountView,
    authority : & AccountView,
    token_program : & AccountView,
    mint : & Address) ->ProgramResult{

    let token_address  = token_program.address();


    let expected_ata = get_ata_address(authority.address(), mint,&token_address);
    if expected_ata.ne(&ata_account.address()){
        return Err(EscrowError::AtaPdaDeriveMismatch.into());
    }

    Ok(())
}

pub fn token_account_init_if_needed<'info>(
    payer : &'info AccountView,
    ata_account : &'info AccountView,
    authority : &'info AccountView,
    token_program : &'info AccountView,
    mint : &'info AccountView,
    system_program : &'info AccountView,
    ) -> ProgramResult
{
    
    if ata_account.lamports() == 0 {
        
        TokenCreate{
            funding_account : payer,
            account : ata_account,
            wallet : authority,
            mint : mint,
            system_program,
            token_program,
        }.invoke()?;
        ata_address_check(ata_account, authority, token_program, mint.address())?;
    } else{
        ata_check(ata_account, authority, token_program, mint.address())?;
    };
    Ok(())
}
pub fn ata_check(
    ata_account : & AccountView,
    authority : & AccountView,
    token_program : & AccountView,
    mint : & Address,
    ) -> ProgramResult
{


    ata_address_check(ata_account, authority, token_program, mint)?;

    if  !ata_account.owned_by(token_program.address()){
        return Err(EscrowError::AtaOwnerMismatch.into());
    }

    match token_program.address() {
        &TOKEN_PROGRAM_ID => {
            let token_account = TokenAccount::from_account_view(ata_account)?;
            if token_account.mint().ne(mint){
                return Err(EscrowError::AtaMintMismatch.into());
            }
            if token_account.owner().ne(authority.address()){
                return Err(EscrowError::AtaOwnerMismatch.into());
            }
        },
        &TOKEN_2022_PROGRAM_ID => {
            let token_account = TokenAccount2022::from_account_view(ata_account)?;
            if token_account.mint().ne(mint){
                return Err(EscrowError::AtaMintMismatch.into());
            }
            if token_account.owner().ne(authority.address()){
                return Err(EscrowError::AtaOwnerMismatch.into());
            }
        },
        _ => {
            return Err(ProgramError::InvalidAccountData);
        }

    }
   
    
    Ok(())
}

pub fn  close_system_account<'info> (account : &'info AccountView, destination : &'info AccountView) -> ProgramResult
{

    // log!("pre close,destination lamports {}, account lamports {} ", destination.lamports(), account.lamports());
    //如果已经关闭，owner是不同的
    if account.lamports() == 0{
        // log!("close_system_account account already closed");
        return Ok( ())
    }
    
    if !account.owned_by(&crate::ID){
        return Err(ProgramError::InvalidAccountOwner);
    }
    
    let lamports = account.lamports().checked_add(destination.lamports()).ok_or(ProgramError::ArithmeticOverflow)?;
    destination.set_lamports(lamports);
    account.set_lamports(0);

    //log!("after set_lamports close,destination lamports {}, account lamports {} ", destination.lamports(), account.lamports());
    account.close()

    

}

pub fn signer_check(signer :&AccountView) -> ProgramResult{
    if !signer.is_signer(){
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

