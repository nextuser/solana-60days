use constant_product_curve::LiquidityPair;
use pinocchio::ProgramResult;
use pinocchio::AccountView;
use pinocchio::cpi::Signer;
use pinocchio::error::ProgramError;
use pinocchio::{sysvars::Sysvar,
    sysvars::clock::Clock};
use pinocchio_token::state::{TokenAccount};
use pinocchio_token::instructions::{Transfer as TokenTransfer};
use crate::ArgType;
use crate::errors::CustomError;
use crate::state::AmmState;
use crate::state::Config;
use constant_product_curve::ConstantProduct;

pub struct Swap<'info>{
    accounts: SwapAccounts<'info>,
    data : SwapData,
}

#[allow(dead_code)]
pub struct SwapAccounts<'info>{
    pub user : &'info AccountView,
    pub user_x_ata : &'info AccountView,
    pub user_y_ata : &'info AccountView,
    pub vault_x : &'info AccountView,
    pub vault_y : &'info AccountView,
    pub config : &'info AccountView,
    pub token_program : &'info AccountView,
}


#[repr(C,packed)]
pub struct SwapData{
    pub is_x : bool,
    pub amount : u64,
    pub min_output : u64,
    pub expiration : i64,
}

impl TryFrom<&[u8]> for SwapData{
    type Error = ProgramError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let is_x = value[0] != 0;
        let amount = u64::from_le_bytes(value[1..9].try_into().unwrap());
        let min_output = u64::from_le_bytes(value[9..17].try_into().unwrap());
        let expiration = i64::from_le_bytes(value[17..25].try_into().unwrap());
        Ok(Self { is_x, amount, min_output, expiration })
    }
}



impl<'info> TryFrom<&'info [AccountView]> for SwapAccounts<'info>{
    type Error = ProgramError;

    fn try_from(value: &'info [AccountView]) -> Result<Self, Self::Error> {
        if value.len() < Swap::ACCOUNT_COUNT {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let user = &value[0];
        let user_x_ata = &value[1];
        let user_y_ata = &value[2];
        let vault_x = &value[3];
        let vault_y = &value[4];
        let config = &value[5];
        let token_program = &value[6];
        Ok(Self { user, user_x_ata, user_y_ata, vault_x, vault_y, config, token_program })
    }
}



impl<'info> TryFrom<ArgType<'info>> for Swap<'info>{
    type Error = ProgramError;

    fn try_from(value: ArgType<'info>) -> Result<Self, Self::Error> {
        let (data, accounts) = value;

        let data = SwapData::try_from(data)?;
        let accounts = SwapAccounts::try_from(accounts)?;
       
       Ok(Self { accounts,data })
        
    }
}


impl<'info> Swap<'info>{
    pub const DISCRIMINATOR: &'info u8  = &3;
    const ACCOUNT_COUNT: usize = std::mem::size_of::<SwapAccounts>()/std::mem::size_of::<AccountView>();

    pub fn process(&self) ->ProgramResult {
        let clock   = Clock::get()?;
        if clock.unix_timestamp > self.data.expiration {
            return Err(CustomError::Expired.into());
        }
        let config = Config::load(self.accounts.config)?;
        let seeds = config.get_seeds();
        let signer = Signer::from(seeds.as_slice());
        
        let (vault_x_amount, vault_y_amount, fee) = {
            if !config.is_match_state(AmmState::Initialized) {
                return Err(CustomError::InvalidAmmState.into());
            }

            let vault_x =  TokenAccount::from_account_view(self.accounts.vault_x)?;
            let vault_y =  TokenAccount::from_account_view(self.accounts.vault_y)?;
            (vault_x.amount(), vault_y.amount(), config.fee())
        };

        let mut curve = ConstantProduct::init(
            vault_x_amount,
            vault_y_amount,
            vault_x_amount,
            fee,
            None,
        ).map_err(|_| ProgramError::ArithmeticOverflow)?;

        let pair = if self.data.is_x {
            LiquidityPair::X
        } else {
            LiquidityPair::Y
        };
        let swap_result = curve.swap(pair, 
            self.data.amount, 
            self.data.min_output).map_err(|_| {CustomError::SwapFailed})?;

        if self.data.is_x {
                    
            TokenTransfer{
                from : self.accounts.user_x_ata,
                to : self.accounts.vault_x,
                authority : self.accounts.user,
                amount : swap_result.deposit,
            }.invoke()?;
            TokenTransfer{
                from : self.accounts.vault_y,
                to : self.accounts.user_y_ata,
                authority : self.accounts.config,
                amount : swap_result.withdraw,
            }.invoke_signed(&[signer])?;
        }
        else
        {
            TokenTransfer{
                from : self.accounts.user_y_ata,
                to : self.accounts.vault_y,
                authority : self.accounts.user,
                amount : swap_result.deposit,
            }.invoke()?;
            TokenTransfer{
                from : self.accounts.vault_x,
                to : self.accounts.user_x_ata,
                authority : self.accounts.config,
                amount : swap_result.withdraw,
            }.invoke_signed(&[signer])?;
        }

        Ok(())
    }
}