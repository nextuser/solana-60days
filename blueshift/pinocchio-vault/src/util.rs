const SEEDS : &[u8] = b"vault";

use pinocchio::pubkey::{Pubkey,find_program_address};

pub fn derive_vault_address(authority : &Pubkey) -> (Pubkey,u8) {
    let (vault,bump)= find_program_address(
            &[SEEDS,authority.as_ref()], 
            &crate::ID);
    return (vault,bump);
    

}
