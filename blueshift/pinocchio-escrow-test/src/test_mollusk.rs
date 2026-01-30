// use mollusk_svm::{
//         Mollusk,
//         sysvar::{self,Sysvars},
//     };
    
//  use   solana_sdk::{
//         account::Account, 
//         pubkey::Pubkey, 
//         signer::{Signer,keypair::Keypair},
//     };

// use tokio;

// use super::util::system_account_with_lamports;
// use solana_sdk::program_pack::Pack;
// use solana_system_interface::instruction::{self as system_instruction,create_account};

// use spl_token_interface::{
//     ID as TokenProgramId,
//     state::Mint,

//     instruction::{initialize_mint2, mint_to},
// };
// use spl_token;
// use anyhow::Result;

// #[test]
// pub fn test_mollusk() -> Result<()>{ 
//     let mollusk = Mollusk::default();
//     mollusk.add(spl_token::ID,&spl_token::processor::Processor::default());
//     let alice = Keypair::new();
//     let bob = Keypair::new();
//     let authority = Keypair::new();
//     let authority_key = authority.pubkey();
//     let mint_a = Keypair::new().pubkey();
//     let mint_b = Keypair::new().pubkey();
//     let mollusk = Mollusk::default();
//     let space = Mint::LEN ;
//     let vars = sysvar::Sysvars::default();
    
//     let mint_rent_fee = vars.rent.minimum_balance(space);

//     let create_mint_a_ins = system_instruction::create_account(
//         &mint_a, 
//         &authority_key, 
//         mint_rent_fee, 
//         space as u64 ,
//         &TokenProgramId,
//         );

//     let mint_a_ins = initialize_mint2(
//         &TokenProgramId,
//         &mint_a,
//         &authority_key,
//         None,
//         6
//     )?;

//     let create_mint_b_ins = system_instruction::create_account(

//         &mint_b, 
//         &authority_key, 
//         mint_rent_fee, 
//         space as u64 ,
//         &TokenProgramId,
//         );
//     let mint_b_ins = initialize_mint2(
//         &TokenProgramId,
//         &mint_b,
//         &authority_key,
//         None,
//         6
//     )?;

//     let instructions = [create_mint_a_ins, create_mint_b_ins, mint_a_ins, mint_b_ins];
//     const LAMPORTS: u64 = 100000000000;
//     let result = mollusk.process_instruction_chain(
//         &instructions,
//         &[
//             ( mint_a,system_account_with_lamports(LAMPORTS)), 
//             ( mint_b, system_account_with_lamports(LAMPORTS)), 
//             ( authority_key, system_account_with_lamports(LAMPORTS))
//         ]
//     );
//     println!("Result:{:#?}", result);
//     Ok(())
    
// }