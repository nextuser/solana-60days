use pinocchio_token::{
    instructions::Transfer as TokenTransfer
};

use pinocchio::{
    AccountView, ProgramResult,
    cpi::{Signer,Seed}, 
    error::ProgramError, 
    sysvars::{Sysvar, rent::Rent}
   
};
use crate::{ata_check, instructions::util::mint_check};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_associated_token_account::instructions::{Create as CreateTokenAccount};
//use crate::instructions::util::create_token_account;
use crate::{Escrow};

#[allow(dead_code)]
pub struct MakeAccounts <'a>{
    maker : &'a AccountView,
    escrow : &'a AccountView,
    mint_a : &'a AccountView,
    mint_b : &'a AccountView,
    maker_ata_a : &'a AccountView,
    
    vault_a : &'a AccountView,
    token_program : &'a AccountView,
    system_program : &'a AccountView,
    
}

pub struct MakeData{
    seed : u64,
    amount : u64,
    receive : u64,
}

#[test]
pub fn test_data(){
    println!("Make data len: {}",std::mem::size_of::<MakeData>());
}

pub struct Make<'a>{
    data : MakeData,
    accounts : MakeAccounts<'a>,
}

impl<'info>  Make<'info> {
    pub const DISCRIMINATOR : &'info u8 = &0;

    pub fn process(&self) -> ProgramResult
    {
        let (escrow_pda,bump) = Escrow::derive_escrow_pda(self.accounts.maker.address(), self.data.seed);
        if self.accounts.escrow.address() != &escrow_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        let space = Escrow::SIZE ;
        let rent_fee = Rent::get()?.try_minimum_balance(space)?;
        let bump_binding = [bump];
        let seed_binding = self.data.seed.to_le_bytes();
        let seeds = [
            Seed::from(Escrow::SEED_PREFIX),
            Seed::from(self.accounts.maker.address().as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&bump_binding)
        ];
        let signer = Signer::from(&seeds);

        CreateAccount{
            from : self.accounts.maker,
            to : self.accounts.escrow,
            lamports : rent_fee,
            space : space as u64,
            owner : &crate::ID,

        }.invoke_signed(&[signer])?;


       CreateTokenAccount{
            funding_account : self.accounts.maker,
            account : self.accounts.vault_a,
            wallet : self.accounts.escrow,
            mint : self.accounts.mint_a,
            system_program : self.accounts.system_program,
            token_program : self.accounts.token_program,
        }.invoke()?;


        TokenTransfer{
            from : self.accounts.maker_ata_a,
            to:self.accounts.vault_a,
            authority:self.accounts.maker,
            amount : self.data.amount,

        }.invoke()?;


        let mut escrow_data_arr_= self.accounts.escrow.try_borrow_mut()?;
        let bytes = escrow_data_arr_.as_mut();
        let escrow: &mut Escrow = Escrow::load_mut( bytes)?;
        
        let maker = self.accounts.maker.address().clone();
        let mint_a = self.accounts.mint_a.address().clone();
        let mint_b = self.accounts.mint_b.address().clone();
        let receive = self.data.receive;

        escrow.maker = maker;
        escrow.mint_a = mint_a;
        escrow.mint_b = mint_b;

        escrow.receive = receive;
        escrow.bump = bump;
        escrow.seed = self.data.seed;
        Ok(())
    }
}

use crate::ArgType;

//use pinocchio_associated_token_account::derive_ata_pda;



// pub fn derive_vault_pda(mint : AccountView) -> (Address,u8) {
//     const SEEDS : &[u8] = b"vault";
//     derive_ata_pda()
//     Address::find_program_address(&[SEEDS,mint.address().as_ref()], &crate::ID)
// }
impl <'info>  TryFrom<ArgType<'info>> for Make<'info>{
    type Error = ProgramError;
    fn try_from(arg: ArgType<'info>) -> Result<Self, Self::Error> {
        let (data, accounts) = arg;
        if data.len() < 24 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let seed = u64::from_le_bytes(data[0..8].try_into().map_err(|_| ProgramError::InvalidInstructionData)?);
        let receive = u64::from_le_bytes(data[8..16].try_into().map_err(|_| ProgramError::InvalidInstructionData)?);
        let amount = u64::from_le_bytes(data[16..24].try_into().map_err(|_| ProgramError::InvalidInstructionData)?);

        let [    
            maker ,
            escrow ,
            mint_a ,
            mint_b ,
            maker_ata_a ,
            
            vault_a ,
            system_program ,
            token_program,
            _
            
             ] = accounts else {
                 return Err(ProgramError::NotEnoughAccountKeys);
            };


        if !maker.is_signer() {
            return Err(ProgramError::InvalidAccountData);
        }

        if !escrow.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }
        if !vault_a.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }

        mint_check(mint_a, token_program)?;
        mint_check(mint_b, token_program)?;
        ata_check(maker_ata_a, 
            maker, 
            token_program,
            mint_a.address()            
            )?;
        // ata_check(vault_a, 
        //     escrow,
        //     token_program,
        //     mint_a          
        //     )?;
        
        Ok(Self{
            data : MakeData { seed, amount, receive },
            accounts : MakeAccounts{
                maker ,
                escrow ,
                mint_a ,
                mint_b ,
                maker_ata_a ,
                vault_a ,
                system_program ,
                token_program ,
            },
        })
    }
}

