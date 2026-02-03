use pinocchio::{AccountView,ProgramResult,Address};
use solana_account_view::{Ref,RefMut};
use pinocchio_associated_token_account;
use pinocchio_token_2022::{ID as TOKEN_2022_PROGRAM_ID,state::TokenAccount as TokenAccount2022};
use pinocchio::error::ProgramError;
use pinocchio_token::{ID as TOKEN_PROGRAM_ID, state::{Mint}};
   
use crate::errors::CustomError;
#[inline(always)]
pub unsafe fn from_bytes_unchecked<T>(bytes : &[u8]) -> & T{
    &*(bytes.as_ptr() as * const T)
}

#[inline(always)]
pub unsafe fn from_bytes_unchecked_mut<T>(bytes : &mut [u8]) -> &mut T{
    &mut *(bytes.as_mut_ptr() as * mut T)
}


#[inline(always)]

 pub fn load_mut<'info,T> (account : &'info  AccountView)-> Result<RefMut<'info,T>, ProgramError>{
    if core::mem::size_of::<T>() != account.data_len() {
        return Err(ProgramError::InvalidAccountData);
    }

    if account.owned_by(&crate::ID){
        return Err(ProgramError::InvalidAccountOwner);
    }



   Ok(RefMut::map(
    account.try_borrow_mut()?, 
    | t| unsafe { from_bytes_unchecked_mut(t) }
    ))

 }

 pub fn load<'info,T> (account : &'info  AccountView)-> Result<Ref<'info,T>, ProgramError>{
    if core::mem::size_of::<T>() != account.data_len() {
        return Err(ProgramError::InvalidAccountData);
    }

    if account.owned_by(&crate::ID){
        return Err(ProgramError::InvalidAccountOwner);
    }

    Ok(Ref::map(account.try_borrow()?, |bytes| unsafe  {
        from_bytes_unchecked::<T>(bytes)
    }))
 }

 pub fn ata_address_check(
    ata_address : &Address,
    authority : &Address, 
    mint_address : &Address, 
    token_program : &Address) -> ProgramResult{
    let (expected_ata_address,_bump) = Address::find_program_address(
        &[
            authority.as_ref(),
            token_program.as_ref(), 
            mint_address.as_ref()], 
            &pinocchio_associated_token_account::ID);

    if ata_address.ne(&expected_ata_address) {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
 }

#[inline(always)]
 pub fn check_signer(account : &AccountView) -> ProgramResult{
    if !account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
 }

// #[inline(always)]
//  pub fn program_check(pda : & AccountView, space : usize) -> ProgramResult
// {
//     if  !pda.owned_by(&crate::ID) {
//         return Err(CustomError::PdaOwnerMismatch.into());
//     }

//     if pda.data_len() != space{
//         return Err(CustomError::PdaSpaceMismatch.into());
//     }
//     Ok(())
// }

#[inline(always)]
pub fn mint_check_by_token_program(
    mint : & AccountView,
    token_program : & AccountView,
    ) -> ProgramResult
{
    if !mint.owned_by(token_program.address()){
        return Err(CustomError::MintOwnerMismatch.into());
    }
    let token_address = token_program.address();
    
    match token_address {
        &TOKEN_PROGRAM_ID => {
            
            if mint.data_len() != Mint::LEN{
                return Err(CustomError::MintSpaceMismatch.into());
            }
        },
        &TOKEN_2022_PROGRAM_ID => {
            if mint.data_len() < TokenAccount2022::BASE_LEN{
                return Err(CustomError::MintSpaceMismatch.into());
            }
            
        },
        _ => {
            return Err(CustomError::InvalidTokenProgram.into());
        }
    }
    Ok(())
}


// #[inline(always)]
// pub fn mint_check(
//     mint : & AccountView,
//     ) -> ProgramResult
// {
//     if !mint.owned_by(&TOKEN_PROGRAM_ID){
//         return Err(CustomError::MintOwnerMismatch.into());
//     }
//     if mint.data_len() != Mint::LEN{
//         return Err(CustomError::MintSpaceMismatch.into());
//     }
    
   
//     Ok(())
// }