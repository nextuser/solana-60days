use core::mem::size_of;
use solana_account_view::{Ref,RefMut};
use pinocchio::{
    AccountView,
    error::ProgramError,
    cpi::{ Seed}
};

use crate::errors::CustomError;

#[repr(C)]
pub struct Config {
    pub state : u8,
    pub seed : [u8;8],
    pub fee : u16,
    pub mint_x : [u8; 32],
    pub mint_y : [u8; 32],
    pub config_bump : [u8;1],
    pub lp_bump : [u8;1],   
    authority : [u8;32],
    
}

pub const SEED_CONFIG_PREFIX : &[u8] = b"config";
pub const SEED_MINT_LP_PREFIX : &[u8] = b"mint_lp";

#[test]
fn test_space_arr(){
    println!("arr size of [u8;1]: {}", size_of::<[u8; 1]>());
}

pub const PRECISION: u32 = 10u32.pow(6);

#[repr(u8)]
pub enum AmmState{
    Uninitialized = 0u8,
    Initialized = 1u8,
    Disable = 2u8,
    WithdrawOnly = 3u8,

}

#[allow(unused_variables)]
impl TryFrom<u8> for AmmState{
    type Error = ProgramError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value{
            0 => Ok(Self::Uninitialized),
            1 => Ok(Self::Initialized),
            2 => Ok(Self::Disable),
            3 => Ok(Self::WithdrawOnly),
            _ => Err(CustomError::InvalidAmmState.into()),
        }
    }
}


impl<'info> Config{
    pub const LEN : usize = size_of::<Config>();

    pub fn load(account : &'info AccountView) -> Result<Ref<'info, Self>, ProgramError> {
        crate::helper::load::<Config>(account)

    }

    pub fn load_mut(account : &'info AccountView) -> Result<RefMut<'info, Self>, ProgramError> {
        crate::helper::load_mut::<Config>(account)
    }

    pub fn set_inner(&mut self,seed :u64, fee : u16, mint_x : [u8; 32], mint_y : [u8; 32], config_bump : [u8;1], lp_bump : [u8;1], authority :[u8;32]) {
        self.state = AmmState::Initialized as u8;
        self.authority = authority;
        self.mint_x = mint_x;
        self.mint_y = mint_y;
        self.seed = seed.to_be_bytes();
        self.fee = fee;
        self.lp_bump = lp_bump;
        self.config_bump = config_bump;
        self.authority = authority;
    }

    pub fn is_match_state(&self,state :AmmState) -> bool {
        state as u8  == self.state
    }
    
 
    pub fn get_state(&self) -> Result<AmmState,ProgramError> {
        self.state.try_into()
    }

    pub fn seed(&self) -> u64 {
        u64::from_le_bytes(self.seed)
    }
    pub fn fee(&self) -> u16 {
        self.fee
    }

    pub fn authority(&self) -> &[u8;32] {
        &self.authority
    }

    pub fn mint_x(&self) -> &[u8;32] {
        &self.mint_x
    }
    pub fn mint_y(&self) -> &[u8;32] {
        &self.mint_y
    }

    pub fn get_seeds(& self) -> [Seed<'_> ;5] {
        let seeds = [
            Seed::from(SEED_CONFIG_PREFIX),
            Seed::from(&self.seed),
            Seed::from(&self.mint_x),
            Seed::from(&self.mint_y),
            Seed::from(&self.config_bump),
        ];

        seeds
    }

}




#[test]
pub fn test_address_len(){
    use pinocchio::Address;
    assert_eq!(size_of::<Address>(),size_of::<[u8;32]>());
}

