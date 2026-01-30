#[cfg(test)]
use pinocchio_token::state::{TokenAccount,Mint};
use pinocchio_token_2022::state::{Mint as Mint2022};

mod tests { 
    use super::*;
    #[test]
    fn test_refund_accounts_size() {
        println!("token account size: {}", std::mem::size_of::<TokenAccount>());
        println!("mint len {}, mint 2022 len {}", Mint::LEN, Mint2022::BASE_LEN);
    }

}