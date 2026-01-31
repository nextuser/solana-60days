
use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
    cpi::{Seed,Signer}
};
use crate::{errors::EscrowError, instructions::{ mint_check, program_check}};
use crate::instructions::ata_check;
use crate::instructions::signer_check;
use super::escrow::Escrow;
use super::util::{token_account_init_if_needed,close_system_account};
use pinocchio_token::{
    instructions::{CloseAccount as CloseTokenAccount, TransferChecked},
    state::{Mint, TokenAccount}
    };
use solana_program_log::log;

#[allow(dead_code)]
pub struct TakeAccounts<'a>{
  pub taker: &'a AccountView,
  pub maker: &'a AccountView,
  pub escrow: &'a AccountView,
  pub mint_a: &'a AccountView,
  pub mint_b: &'a AccountView,
  pub vault_a: &'a AccountView,
  pub taker_ata_a: &'a AccountView,
  pub taker_ata_b: &'a AccountView,
  pub maker_ata_b: &'a AccountView,
  pub system_program: &'a AccountView,
  pub token_program: &'a AccountView,
}
pub struct Take<'a>{
    accounts : TakeAccounts<'a>,
}

impl <'info> Take<'info>{
    pub const DISCRIMINATOR : &'info u8 = &1;

    pub fn process(&self) -> ProgramResult
    {
        let accounts = &self.accounts;

        let (receive,seed,bump) ={
            let escrow  = Escrow::from_account_view(accounts.escrow)?;
            if escrow.maker.ne(accounts.maker.address()) {
                return Err(EscrowError::EscrowMakerMismatch.into());
            }


            if escrow.mint_a.ne(accounts.mint_a.address()) {
                return Err(EscrowError::EscrowMintAMismatch.into());
            }

            if escrow.mint_b.ne(accounts.mint_b.address()) {
                return Err(EscrowError::EscrowMintBMismatch.into());
            }
            (escrow.receive,escrow.seed,escrow.bump)
        };


        let decimals_b = Mint::from_account_view(accounts.mint_b)?.decimals();
        let decimals_a = Mint::from_account_view(accounts.mint_a)?.decimals();

        let vault_amount = TokenAccount::from_account_view(accounts.vault_a)?.amount();


        let seed_binding = seed.to_le_bytes();
        let bump_binding = [bump];
        // vault_a => taker_ata_a
        let seeds = [
            Seed::from(crate::Escrow::SEED_PREFIX),
            Seed::from(accounts.maker.address().as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&bump_binding),
        ];
        let signer = Signer::from(&seeds);

        TransferChecked{
            mint : accounts.mint_b,
            from : accounts.taker_ata_b,
            to :   accounts.maker_ata_b,
            authority : accounts.taker,
            amount : receive,
            decimals :decimals_b,
        }.invoke()?;       
        
        
        TransferChecked{
            mint : accounts.mint_a,
            from : accounts.vault_a,
            to : accounts.taker_ata_a,
            authority : accounts.escrow,
            amount : vault_amount,
            decimals : decimals_a,
        }.invoke_signed(&[signer.clone()] )?;
        
        // let seed_binding = escrow.seed.to_le_bytes();
        // let escrow_seeds = [
        //     Seed::from(b"escrow"),                           // 种子 1: "escrow"
        //     Seed::from(accounts.maker.address().as_ref()),  // 种子 2: maker 地址
        //     Seed::from(&seed_binding),                       // 种子 3: seed 的字节数组
        //     Seed::from(&escrow.bump),                       // 种子 4: bump
        // ];

        // // 创建 PDA 签名者
        // // 对应 Anchor 的 &signer_seeds 参数
        // let signer = Signer::from(&escrow_seeds);

        CloseTokenAccount{
             account: accounts.vault_a,
             destination: accounts.maker,
             authority: accounts.escrow,
        }.invoke_signed (&[signer] )?;
        // log!("after close CloseTokenAccount");
        close_system_account(accounts.escrow, accounts.maker)?;
        // log!("after close System account ");
        Ok( ())
        
    }


}


impl <'info> TryFrom< &'info [AccountView]> for Take<'info>{ 
    type Error = ProgramError;
    fn try_from(accounts: &'info [AccountView]) -> Result<Self, Self::Error> { 

        let [
                taker,
                maker,
                escrow,
                mint_a,
                mint_b,
                vault_a,
                taker_ata_a,
                taker_ata_b,
                maker_ata_b,
                system_program,
                token_program,
                _
                 
        ] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        
        signer_check(taker)?;
        program_check(escrow, Escrow::SIZE)?;

        // taker_ata_b => maker_ata_b
        mint_check(mint_a, token_program)?;
        mint_check(mint_b, token_program)?;
        ata_check(taker_ata_b, taker,token_program,mint_b.address())?;
        ata_check(vault_a, escrow,token_program,mint_a.address())?;
        token_account_init_if_needed(taker,taker_ata_a, taker, token_program, mint_a,system_program)?;
        token_account_init_if_needed(taker,maker_ata_b, maker, token_program, mint_b,system_program)?;


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
                token_program ,
                system_program 
                
            }
        })
    }
    

}