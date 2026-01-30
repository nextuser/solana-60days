
use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
    cpi::{Seed,Signer}
};
use crate::instructions::{ mint_check, program_check};
use crate::instructions::ata_check;
use crate::instructions::signer_check;
use super::escrow::Escrow;
use super::util::{token_account_init_if_needed};
use pinocchio_token::{
    instructions::{TransferChecked,CloseAccount as CloseTokenAccount},
    state::Mint
    };

#[allow(dead_code)]
pub struct TakeAccounts<'a>{
    taker : &'a AccountView,
    maker : &'a AccountView,// no need use
    escrow : &'a AccountView,

    mint_a : &'a AccountView,
    mint_b : &'a AccountView,
    
    vault_a : &'a AccountView,

    taker_ata_a : &'a AccountView,
    taker_ata_b : &'a AccountView,  
    maker_ata_b : &'a AccountView,  
    system_program : &'a AccountView,
    token_program : &'a AccountView,
}
pub struct Take<'a>{
    accounts : TakeAccounts<'a>,
}

impl <'info> Take<'info>{
    pub const DISCRIMINATOR : &'info u8 = &1;

    pub fn process(&self) -> ProgramResult
    {
        let accounts = &self.accounts;
        let escrow  = Escrow::from_account_view(accounts.escrow)?;
        if escrow.maker.ne(accounts.maker.address()) {
            return Err(ProgramError::InvalidAccountData);
        }


        if escrow.mint_a.ne(accounts.mint_a.address()) {
            return Err(ProgramError::InvalidAccountData);
        }

        if escrow.mint_b.ne(accounts.mint_b.address()) {
            return Err(ProgramError::InvalidAccountData);
        }


        let mint_info_b = Mint::from_account_view(accounts.mint_b)?;
        TransferChecked{
            mint : accounts.mint_b,
            from : self.accounts.taker_ata_b,
            to:self.accounts.maker_ata_b,
            authority:self.accounts.taker,
            amount : escrow.receive,
            decimals : mint_info_b.decimals(),
        }.invoke()?;
        let seed_binding = escrow.seed.to_le_bytes();
        // vault_a => taker_ata_a
        let seeds = [
            Seed::from(crate::Escrow::SEED),
            Seed::from(escrow.maker.as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&escrow.bump),
        ];
        let signer = Signer::from(&seeds);
        let mint_info_a = Mint::from_account_view(accounts.mint_a)?;
        
        TransferChecked{
            mint : accounts.mint_a,
            from : self.accounts.vault_a,
            to:self.accounts.taker_ata_a,
            authority:self.accounts.escrow,
            amount : escrow.amount,
            decimals : mint_info_a.decimals(),
        }.invoke_signed(&[signer.clone()] )?;
        
        // let seed_binding = escrow.seed.to_le_bytes();
        // let escrow_seeds = [
        //     Seed::from(b"escrow"),                           // 种子 1: "escrow"
        //     Seed::from(self.accounts.maker.address().as_ref()),  // 种子 2: maker 地址
        //     Seed::from(&seed_binding),                       // 种子 3: seed 的字节数组
        //     Seed::from(&escrow.bump),                       // 种子 4: bump
        // ];

        // // 创建 PDA 签名者
        // // 对应 Anchor 的 &signer_seeds 参数
        // let signer = Signer::from(&escrow_seeds);

        CloseTokenAccount{
             account: self.accounts.vault_a,
             destination: self.accounts.maker,
             authority: self.accounts.escrow,
        }.invoke_signed (&[signer] )?;
        
        Ok( ())


        
    }


}


impl <'info> TryFrom< &'info [AccountView]> for Take<'info>{ 
    type Error = ProgramError;
    fn try_from(accounts: &'info [AccountView]) -> Result<Self, Self::Error> { 

        let [
                taker ,
                maker ,
                escrow ,

                mint_a ,
                mint_b ,
                
                vault_a ,

                taker_ata_a ,
                taker_ata_b ,  
                maker_ata_b ,  
                system_program ,
                token_program 
        ] = accounts else {
            return Err(ProgramError::InvalidAccountData);
        };
        
        signer_check(taker)?;
        program_check(escrow, Escrow::SIZE)?;

                // taker_ata_b => maker_ata_b
        mint_check(mint_a, token_program)?;
        mint_check(mint_b, token_program)?;
        ata_check(taker_ata_b, taker,token_program,mint_b)?;
        ata_check(vault_a, escrow,token_program,mint_a)?;
        token_account_init_if_needed(taker,taker_ata_a, taker, token_program, mint_a,system_program)?;
        token_account_init_if_needed(taker,maker_ata_b, maker, token_program, mint_b,system_program)?;

        // let escrow_data= Escrow::from(escrow);
        // if escrow_data.maker.ne( maker.address()){
        //     return Err(ProgramError::InvalidAccountData);
        // }
        // if escrow_data.mint_a.ne( mint_a.address()){
        //     return Err(ProgramError::InvalidAccountData);
        // }

        // if escrow_data.mint_b.ne( mint_b.address()){
        //     return Err(ProgramError::InvalidAccountData);
        // }

 

        

        Ok(Take {
            accounts: TakeAccounts{
                taker ,
                maker ,
                escrow ,
                mint_a ,
                mint_b ,
                vault_a ,
                taker_ata_a ,
                taker_ata_b ,  
                maker_ata_b ,  
                system_program ,
                token_program 
            }
        })
    }
    

}