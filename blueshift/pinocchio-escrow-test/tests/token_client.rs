use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
    sysvar::rent::Rent,
    
    //system_instruction::{},
};
use solana_system_interface::{
    instruction::create_account,
};
use mollusk_svm::Mollusk;
use anyhow::Result;

async fn create_mint_account(
    svm : Mollusk,
    payer :         & dyn Signer,
    mint_key:Pubkey,
    authority : Pubkey,
) -> Result<()> {



    Ok(())
}
