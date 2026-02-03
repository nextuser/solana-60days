use pinocchio::ProgramResult;
use pinocchio::{ Address,AccountView};
use pinocchio::error::ProgramError;
use pinocchio_token::state::{Mint,TokenAccount};
use pinocchio_token::instructions::{Transfer,MintTo};
use constant_product_curve::ConstantProduct  as ConstantProductCurve;
use crate::ArgType;
use crate::errors::CustomError;
use crate::helper;
use crate::state::{AmmState,Config};
use solana_program_log::log;
use pinocchio::{
    sysvars::{
        Sysvar,
        clock::Clock
    },
    cpi::Signer,
};
    

pub struct Deposit<'info>{
    accounts: DepositAccounts<'info>,
    instruction_data : DepositInstructionData,
}

#[repr(C,packed)]
pub struct DepositInstructionData{
    pub amount : u64,
    pub max_x : u64,
    pub max_y : u64,
    pub expiration : i64,
}

impl TryFrom<&[u8]> for DepositInstructionData{
    type Error = ProgramError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != std::mem::size_of::<Self>(){
            return Err(CustomError::InvalidDepositData.into());
        }
        let amount = u64::from_le_bytes(value[0..8].try_into().unwrap());
        let max_x = u64::from_le_bytes(value[8..16].try_into().unwrap());//.try_into().map_err(|_| ProgramError::InvalidDepositData)?);
        let max_y = u64::from_le_bytes(value[16..24].try_into().unwrap());//.try_into().map_err(|_| ProgramError::InvalidDepositData)?);    
        let expiration = i64::from_le_bytes(value[24..32].try_into().unwrap());
        Ok(Self { amount, max_x, max_y, expiration })
    }
}

pub struct DepositAccounts<'info>{
    pub user : &'info AccountView,
    pub mint_lp : &'info AccountView,
    pub vault_x : &'info AccountView,
    pub vault_y : &'info AccountView,
    pub user_x_ata : &'info AccountView,
    pub user_y_ata : &'info AccountView,
    pub user_lp_ata : &'info AccountView,
    pub config : &'info AccountView,
    pub token_program : &'info AccountView,
}

impl<'info> DepositAccounts<'info>{
    pub const ACCOUNT_COUNT: usize = std::mem::size_of::<DepositAccounts>()/std::mem::size_of::<AccountView>();
}



impl <'info> TryFrom<&'info [AccountView]> for DepositAccounts<'info>{
    type Error = ProgramError;
    fn try_from(accounts: &'info [AccountView]) -> Result<Self, ProgramError> {
        if accounts.len() < DepositAccounts::ACCOUNT_COUNT {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        let mut iter = accounts.iter();
        // let [user,mint_lp,vault_x,vault_y,user_x_ata,user_y_ata,user_lp_ata,config,system_program] = accounts  else {
        //     return Err(ProgramError::InvalidInstructionData);
        // };
        let user = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
        let mint_lp = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
        let vault_x = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
        let vault_y = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
        let user_x_ata = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
        let user_y_ata = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
        let user_lp_ata = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
        let config = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
        let token_program = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

        helper::check_signer(user)?;
        helper::mint_check_by_token_program(mint_lp, token_program)?;


        
        Ok(Self { user, mint_lp, vault_x, vault_y, user_x_ata, user_y_ata,user_lp_ata, config, token_program })
    }
}

impl<'info> TryFrom<ArgType<'info>> for Deposit<'info>{
    type Error = ProgramError;

    fn try_from(value: ArgType<'info>) -> Result<Self, Self::Error> {
        let (data, accounts) = value;
        let instruction_data = DepositInstructionData::try_from(data)?;
        let accounts = DepositAccounts::try_from(accounts)?;
       
        Ok(Self { accounts, instruction_data })
        
    }
}


impl<'info> Deposit<'info>{
    pub const DISCRIMINATOR: &'info u8  = &1;
    //const ACCOUNT_COUNT: usize = std::mem::size_of::<DepositAccounts>()/std::mem::size_of::<AccountView>();

    pub fn process(&self) ->ProgramResult {
        let DepositAccounts{user,mint_lp,vault_x,vault_y,user_x_ata,user_y_ata,user_lp_ata,config,token_program} = self.accounts;
        let DepositInstructionData{amount,max_x,max_y,expiration} = self.instruction_data;

        let clock = Clock ::get()?;
        if clock.unix_timestamp > expiration {
            return Err(CustomError::Expired.into());
        }

        let config_info  = helper::load::<Config>(config)?;
        if  !config_info.is_match_state(AmmState::Initialized){
            return Err(CustomError::NotInitialized.into());
        }

        let (x_amount,y_amount,lp_supply,seeds) = {


            log!("1");
            let seeds = config_info.get_seeds();            

                            //.map_err(|_|{CustomError::LoadConfigFailed})?;
            let mint_x : Address = config_info.mint_x().as_slice().try_into().map_err(|_| CustomError::AddressConvertError)?;
            helper::ata_address_check(user_x_ata.address(), user.address(),     &mint_x, token_program.address())?;

            let mint_y : Address = config_info.mint_y().as_slice().try_into().map_err(|_| CustomError::AddressConvertError)?;
            helper::ata_address_check(user_y_ata.address(), user.address(), &mint_y, token_program.address())?;

            log!("2");
            let vault_x_info = TokenAccount::from_account_view(vault_x)
                        .map_err(|_|{CustomError::LoadValutXFailed})?;
    
            let vault_y_info = TokenAccount::from_account_view(vault_y) 
                        .map_err(|_|    CustomError::LoadValutYFailed)?;
            let mint_info = Mint::from_account_view(mint_lp)
                        .map_err(|_|    CustomError::LoadMintFailed)?;
            let lp_supply = mint_info.supply();
            let (x_amount,y_amount) = (vault_x_info.amount(),vault_y_info.amount());
            (x_amount,y_amount,lp_supply,seeds)
        };

        let signer = Signer::from(seeds.as_slice());
        


        let (x,y) =if lp_supply == 0 {
            (max_x,max_y)
        } else{
            //let const_k = vault_x.amount * vault_y.amount;
            let amounts: constant_product_curve::XYAmounts = ConstantProductCurve::xy_deposit_amounts_from_l(
                x_amount,
                y_amount,
                lp_supply,
                amount,
                crate::PRECISION
            ).map_err(|_|{ProgramError::ArithmeticOverflow})?;
            (amounts.x,amounts.y)
        };
        log!("4");
        if x > max_x  || y > max_y {
            return Err(CustomError::XYExceedMax.into());
        };

        log!("5");

        Transfer{
            from : user_x_ata,
            to : vault_x,
            authority : user,
            amount : x,
        }.invoke()?;
        log!("6");
        Transfer{
            from : user_y_ata,
            to : vault_y,
            authority : user,
            amount : y,
        }.invoke()?;        
        log!("7");
        MintTo{
            mint: mint_lp,
            account : user_lp_ata,
            mint_authority : config,
            amount : amount,
        }.invoke_signed(&[signer])?;
        log!("8");
        Ok(())
    }
}



