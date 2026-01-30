use solana_sdk::account::Account;
pub fn system_account_with_lamports(lamports: u64) -> Account {
    Account::new(lamports, 0, &solana_sdk_ids::system_program::id())
}

