use constant_product_curve::ConstantProduct;
use pinocchio::ProgramResult;
use pinocchio::AccountView;
use pinocchio::error::ProgramError;
use pinocchio::{sysvars::Sysvar,
    sysvars::clock::Clock,
    cpi::Signer,
};
use pinocchio_token::state::{TokenAccount,Mint};
use pinocchio_token::instructions::{Burn,Transfer as TokenTransfer};
use crate::ArgType;
use crate::errors::CustomError;
use crate::helper;
use crate::state::{Config,AmmState};
#[repr(C,packed)]
pub struct WithdrawData{
    amount : u64,
    min_x : u64,
    min_y : u64,
    expiration : i64,
}

impl TryFrom<&[u8]> for WithdrawData{
    type Error = ProgramError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != std::mem::size_of::<WithdrawData>() {
            return Err(CustomError::InvaidWithdrawData.into());
        }
        let amount = u64::from_le_bytes(value[0..8].try_into().unwrap());
        let min_x = u64::from_le_bytes(value[8..16].try_into().unwrap());
        let min_y = u64::from_le_bytes(value[16..24].try_into().unwrap());
        let expiration = i64::from_le_bytes(value[24..32].try_into().unwrap());
        Ok(Self { amount, min_x, min_y, expiration })
    }
}

#[allow(dead_code)]
pub struct WithdrawAccounts<'a>{
    pub user: &'a AccountView,
    pub mint_lp: &'a AccountView,
    pub vault_x: &'a AccountView,
    pub vault_y: &'a AccountView,
    pub user_x_ata: &'a AccountView,
    pub user_y_ata: &'a AccountView,
    pub user_lp_ata: &'a AccountView,
    pub config: &'a AccountView,
    pub token_program: &'a AccountView,
}

impl<'info> TryFrom<&'info [AccountView]> for WithdrawAccounts<'info>{
    type Error = ProgramError;

    fn try_from(value: &'info [AccountView]) -> Result<Self, Self::Error> {
        if value.len() < Withdraw::ACCOUNT_COUNT {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let user = &value[0];
        let mint_lp = &value[1];
        let vault_x = &value[2];
        let vault_y = &value[3];
        let user_x_ata = &value[4];
        let user_y_ata = &value[5];
        let user_lp_ata = &value[6];
        let config = &value[7];
        let token_program = &value[8];
        Ok(Self { user, mint_lp, vault_x, vault_y, user_x_ata, user_y_ata, user_lp_ata, config, token_program })
    }
}

pub struct Withdraw<'info>{
    accounts:  WithdrawAccounts<'info>,
    data : WithdrawData,
}





impl<'info> TryFrom<ArgType<'info>> for Withdraw<'info>{
    type Error = ProgramError;

    fn try_from(value: ArgType<'info>) -> Result<Self, Self::Error> {
        let (data, accounts) = value;
        let data = WithdrawData::try_from(data)?;
        let accounts = WithdrawAccounts::try_from(accounts)?;
        
       
        Ok(Self { accounts,data })
        
    }
}


impl<'info> Withdraw<'info>{
    pub const DISCRIMINATOR: &'info u8  = &2;
    const ACCOUNT_COUNT: usize = std::mem::size_of::<WithdrawAccounts>()/std::mem::size_of::<AccountView>();

    pub fn process(&self) ->ProgramResult {
        let clock = Clock::get()?;
        if clock.unix_timestamp < self.data.expiration {
            return Err(CustomError::WithdrawExpired.into());
        }

        let config = Config::load(self.accounts.config)?;

        if !config.is_match_state(AmmState::Initialized) {
            return Err(CustomError::NotInitialized.into());
        }

        let mint_lp = helper::load::<Mint>(self.accounts.mint_lp)?;
        let lp_supply = mint_lp.supply();
        let valut_x = helper::load::<TokenAccount>(self.accounts.vault_x)?;
        let valut_y = helper::load::<TokenAccount>(self.accounts.vault_y)?;

        let (x,y) = if mint_lp.supply() == self.data.amount {
            (valut_x.amount(),valut_y.amount())
        } else {
            let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
                valut_x.amount(), 
                valut_y.amount(), 
                lp_supply, 
                self.data.amount, 
                crate::state::PRECISION).map_err(|_| CustomError::XYExceedMax)?;
            (amounts.x,amounts.y)
        };

        if x < self.data.min_x || y < self.data.min_y {
            return Err(CustomError::WithdrawAmountTooSmall.into());
        }

        Burn{
            mint : self.accounts.mint_lp,
            account : self.accounts.user_lp_ata,
            authority : self.accounts.user,
            amount : self.data.amount,
        }.invoke()?;
        let seeds = config.get_seeds();
        let signer = Signer::from(seeds.as_slice());
        TokenTransfer{
            from : self.accounts.vault_x,
            to : self.accounts.user_x_ata,
            authority : self.accounts.config,
            amount : x,
        }.invoke_signed(std::slice::from_ref(&signer))?;

        TokenTransfer{
            from : self.accounts.vault_y,
            to : self.accounts.user_y_ata,
            authority : self.accounts.config,
            amount : y,
        }.invoke_signed(&[signer])?;
        Ok(())
    }
}

