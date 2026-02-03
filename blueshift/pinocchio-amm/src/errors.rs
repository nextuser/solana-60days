use pinocchio::error::ProgramError;
use core::fmt;
#[derive(Clone, Copy,Debug)]
pub enum CustomError{
    AtaOwnerMismatch = 0,
    AtaPdaDeriveMismatch = 1,   
    AtaSpaceMismatch = 2,
    AtaMintMismatch = 3,
    MintOwnerMismatch = 4,
    MintSpaceMismatch = 5,
    PdaOwnerMismatch = 6,
    PdaSpaceMismatch = 7,
    InvalidTokenProgram = 8,
    InvalidSystemProgram = 9,
    NotInitialized = 10,
    Expired = 11,
    LoadMintFailed = 12,
    LoadValutXFailed = 13,
    LoadValutYFailed = 14,
    InvalidAmmState = 15,
    XYExceedMax = 16,
    InvaidWithdrawData = 17,
    InvalidDepositData = 18,
    InvalidInitializeData = 19,
    InvalidSwapData = 20,
    WithdrawExpired = 21,
    WithdrawAmountTooSmall = 22,
    AddressConvertError = 23,
    SwapFailed = 24,
}


impl From<CustomError> for ProgramError{

    fn from(value: CustomError) -> ProgramError {
        ProgramError::Custom(value as u32)
    }
} 

impl fmt::Display for CustomError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError: {:?}", self)
    }
}