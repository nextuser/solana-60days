use pinocchio::error::ProgramError;
use core::fmt;


#[derive(Clone,Debug,PartialEq)]
pub enum EscrowError{
    InvalidTokenOwner = 0,
    InvalidEscrowOwner = 1,
    AtaOwnerMismatch = 2,
    
    MintOwnerMismatch = 3,
    AtaSpaceMismatch = 4,
    MintSpaceMismatch = 5,
    PdaOwnerMismatch = 6,
    PdaSpaceMismatch = 7,
    InvalidTokenProgram = 8,
    InvalidSystemProgram = 9,
    AtaMintMismatch = 10,
    EscrowMintAMismatch = 11,
    EscrowMintBMismatch = 12,
    EscrowMakerMismatch = 13,
    AtaPdaDeriveMismatch = 14,
    

}

#[test]
fn test_enum_space(){
    println!("EscrowError space {}", core::mem::size_of::<EscrowError>());
    println!("ProgramError space {}", core::mem::size_of::<ProgramError>());
}

impl From<EscrowError> for ProgramError{
    fn from(error: EscrowError) -> Self {
        ProgramError::Custom(error as u32)
    }
}

impl fmt::Display for EscrowError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EscrowError::InvalidTokenOwner => write!(f, "InvalidTokenOwner"),
            EscrowError::InvalidEscrowOwner => write!(f, "InvalidEscrowOwner"),
            EscrowError::AtaOwnerMismatch => write!(f, "AtaOwnerMismatch"),
            EscrowError::MintOwnerMismatch => write!(f, "MintOwnerMismatch"),
            EscrowError::AtaSpaceMismatch => write!(f, "AtaSpaceMismatch"),
            EscrowError::MintSpaceMismatch => write!(f, "MintSpaceMismatch"),
            EscrowError::PdaOwnerMismatch => write!(f, "PdaOwnerMismatch"),
            EscrowError::PdaSpaceMismatch => write!(f, "PdaSpaceMismatch"),
            EscrowError::InvalidTokenProgram => write!(f, "InvalidTokenProgram"),
            EscrowError::InvalidSystemProgram => write!(f, "InvalidSystemProgram"),
            EscrowError::AtaMintMismatch => write!(f, "AtaMintMismatch"),
            EscrowError::EscrowMintAMismatch => write!(f, "EscrowMintAMismatch"),
            EscrowError::EscrowMintBMismatch => write!(f, "EscrowMintBMismatch"),
            EscrowError::EscrowMakerMismatch => write!(f, "EscrowMakerMismatch"),
            EscrowError::AtaPdaDeriveMismatch => write!(f, "AtaPdaDeriveMismatch"),
        }
    }
}