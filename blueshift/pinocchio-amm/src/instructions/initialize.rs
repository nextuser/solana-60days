use std::mem::MaybeUninit;
use pinocchio::ProgramResult;
use pinocchio::AccountView;
use pinocchio::error::ProgramError;
use pinocchio::{
    sysvars::{rent::Rent,Sysvar},
    cpi::{Signer,Seed},
};
use pinocchio_token::state::{Mint};
use pinocchio_token::instructions::{InitializeMint2};
use pinocchio_system::instructions::{CreateAccount};
use crate::ArgType;
use crate::errors::CustomError;

pub struct InitializeAccounts<'a>{
    pub initializer : &'a AccountView,
    pub mint_lp : &'a AccountView,
    pub config : &'a AccountView,

}


impl<'a> TryFrom<&'a [AccountView]> for InitializeAccounts<'a>{
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        if accounts.len() < Initialize::ACCOUNT_COUNT {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let mut account_iter = accounts.iter();
        let initializer = account_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
        let mint_lp = account_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
        let config = account_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

        crate::helper::check_signer(initializer)?;

        Ok(Self { initializer, mint_lp, config })
    }
}



#[repr(C,packed)]
pub struct InitializeData{

    pub seed : u64,
    pub fee : u16,//106
    pub mint_x : [u8;32],
    pub mint_y : [u8;32],

    pub config_bump : [u8;1],
    pub lp_bump : [u8;1],
    pub authority : [u8;32],

}

#[test]
fn test_initialize_data(){
    println!("initialize data size :{}", std::mem::size_of::<InitializeData>());

    assert_eq!(std::mem::size_of::<[u8;1]>(), std::mem::size_of::<u8>());
}

impl<'a> TryFrom<&'a [u8]> for InitializeData{
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        const LEN_WITHOUT_AUTHORITY : usize = std::mem::size_of::<InitializeData>() - std::mem::size_of::<[u8;32]>();
        const LEN_WITH_AUTHORITY : usize = std::mem::size_of::<InitializeData>();

        match data.len() {
            LEN_WITHOUT_AUTHORITY => {
                let mut raw : MaybeUninit<[u8; LEN_WITH_AUTHORITY]> = MaybeUninit::uninit();
                let raw_ptr = raw.as_mut_ptr() as *mut u8;
                unsafe{
                    core::ptr::copy_nonoverlapping(data.as_ptr(), raw_ptr, LEN_WITHOUT_AUTHORITY);
                    core::ptr::write_bytes(raw_ptr.add(LEN_WITHOUT_AUTHORITY), 0, 32);
                    let idata :InitializeData = (raw.as_ptr() as *const Self).read_unaligned();
                    return Ok(idata);
                }

            },
            LEN_WITH_AUTHORITY => {
                let initialize = 
                    Ok(unsafe{
                        (data.as_ptr() as * const Self).read_unaligned()
                    });
                
                return initialize;
            },
            _ => return Err(CustomError::InvalidInitializeData.into()),  
        }
    }
}



#[test]
fn test_init_space(){
    println!("init space :{}", std::mem::size_of::<InitializeData>());
}
pub struct Initialize<'info>{
    accounts:  InitializeAccounts<'info>,
    data :  InitializeData,
}

impl<'info> TryFrom<ArgType<'info>> for Initialize<'info>{
    type Error = ProgramError;

    fn try_from(value: ArgType<'info>) -> Result<Self, Self::Error> {
        let (data,accounts) = value;
        Ok(Self { 
            accounts: InitializeAccounts::try_from(accounts)?, 
            data: InitializeData::try_from(data)? 
        })
        //Ok(Self { accounts })
    }
}




impl<'info> Initialize<'info>{
    pub const DISCRIMINATOR: &'info u8  = &0;
    const ACCOUNT_COUNT: usize = std::mem::size_of::<InitializeAccounts>()/std::mem::size_of::<AccountView>();

    pub fn process(&self) ->ProgramResult {
        let InitializeAccounts{ initializer, mint_lp, config } = self.accounts;
        let InitializeData{ seed, fee, mint_x, mint_y, config_bump, lp_bump, authority } = self.data;
        let rent = Rent::get()?;
        let config_lamports = rent.try_minimum_balance(std::mem::size_of::<crate::state::Config>())?;
        let seed_binding = seed.to_le_bytes();

        let config_seeds = [
            Seed::from(crate::SEED_CONFIG_PREFIX),
            Seed::from(&seed_binding),
            Seed::from(mint_x.as_ref()),
            Seed::from(mint_y.as_ref()),
            Seed::from(&config_bump)
        ];

        let config_signer = Signer::from(&config_seeds);

        CreateAccount{
            from : initializer,
            to : config,
            lamports : config_lamports,
            space : crate::state::Config::LEN as u64,
            owner : &crate::ID,
        }.invoke_signed(&[config_signer])?;


        
        let mut config_info = crate::state::Config::load_mut(config)?;
        config_info.set_inner(seed, fee, mint_x, mint_y, config_bump, lp_bump, authority);

        let mint_lp_seeds =[
            Seed::from(crate::SEED_MINT_LP_PREFIX),
            Seed::from(config.address().as_ref()),
            Seed::from(&lp_bump),
           // todo
        ];
        let mint_signer = Signer::from(&mint_lp_seeds);

        CreateAccount{
            from : initializer,
            to : mint_lp,
            lamports : rent.try_minimum_balance(Mint::LEN)?,
            space : Mint::LEN as u64,
            owner : &pinocchio_token::ID,
        }.invoke_signed(&[mint_signer])?;

        InitializeMint2{
            mint : mint_lp,
            mint_authority : &config.address(),
            decimals : 6,
            freeze_authority : None,
        }.invoke()?;//居然不要签名

        Ok(())
    }
}

