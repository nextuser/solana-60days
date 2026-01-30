use {
    mollusk_svm::{Mollusk, result::Check}, solana_program::example_mocks::{solana_keypair::Keypair, solana_signer::Signer}, solana_sdk::{account::Account, pubkey::Pubkey}
};
use spl_token_client::{
    token::{
        self,
        instruction::{approve, initialize_account, initialize_mint, mint_to, transfer},
    },
};
fn system_account_with_lamports(lamports: u64) -> Account {
    Account::new(lamports, 0, &solana_sdk_ids::system_program::id())
}


#[test]
pub fn test_mollusk() { 

    let alice = system_account_with_lamports(1_000_000_000);
    let bob = system_account_with_lamports(1_000_000_000);
    let mint_a = Keypair::new().pubkey();
    let mint_b = Keypair::new().pubkey();
    let mollusk = Mollusk::default();




    
}