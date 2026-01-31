use pinocchio::{
    address::Address,
    error::ProgramError,
    account::{AccountView}
};

#[test]
fn test_size(){
    println!("Escrow size: {}", Escrow::SIZE);
}

#[repr(C)]
pub struct Escrow{
    pub seed : u64,
    pub maker : Address,
    pub mint_a : Address,
    pub mint_b : Address,
    pub receive : u64,
    pub bump : u8,
}

#[test]
fn test_space(){
    type Arr1 = [u8;1];
    assert_eq!(core::mem::size_of::<Arr1>(), 1);

    println!("Address size: {}", Escrow::SIZE);
}
use core::mem::size_of;
impl Escrow {
    pub const SEED_PREFIX : &'static [u8] = b"escrow";
    pub const SIZE: usize = size_of::<Address>() * 3 + size_of::<u64>() * 2 + 1;
    //core::mem::size_of::<Escrow>();

    /// Return a `TokenAccount` from the given account view.
    ///
    /// This method performs owner and length validation on `AccountView`, safe borrowing
    /// the account data.
    #[inline]
    pub fn from_account_view(
        account_view: &  AccountView,
    ) -> Result<&Self, ProgramError> {
        if account_view.data_len() != Self::SIZE {
            return Err(ProgramError::InvalidAccountData);
        }
        if !account_view.owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        let data = account_view.try_borrow_mut()?;
        Ok(unsafe { & *core::mem::transmute::<*const u8, *const Self>(data.as_ptr()) })
    }

    pub fn derive_escrow_pda(maker_key:&Address, seed:u64) -> (Address,u8) {
        
        Address::find_program_address(&[
            Self::SEED_PREFIX,
            maker_key.as_ref(),
            seed.to_le_bytes().as_ref()], &crate::ID)
    }


    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        // 验证字节长度是否匹配
        if bytes.len() != Escrow::SIZE {
            return Err(ProgramError::InvalidAccountData);
        }
        // 将字节指针转换为 Escrow 指针，然后解引用为可变引用
        Ok(unsafe { &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) })
    }


    

}


        
#[inline(always)]
pub unsafe fn from_bytes_unchecked<'a,T>(bytes: &'a [u8]) -> &'a T {
    &*(bytes.as_ptr() as *const T)
}

#[inline(always)]
pub unsafe fn from_mut_bytes_unchecked<'a,T>(bytes: &'a mut [u8]) -> &'a mut T {
    &mut *(bytes.as_ptr() as *mut T)
}


impl<'info> TryFrom<&'info [u8]> for &'info  Escrow {
    type Error = ProgramError;
    fn try_from(bytes: &'info [u8]) -> Result< Self, Self::Error> {
        if bytes.len() !=  core::mem::size_of::<Self>() {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { from_bytes_unchecked::<Self>(bytes) })
    }
}




impl<'info> TryFrom<&'info mut [u8]> for &'info mut Escrow {
    type Error = ProgramError;
    fn try_from(bytes: &'info mut [u8]) -> Result< Self, Self::Error> {
        if bytes.len() !=  core::mem::size_of::<Self>() {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { from_mut_bytes_unchecked::<Self>(bytes) })
    }
}



