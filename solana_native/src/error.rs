use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq, FromPrimitive)]
pub enum MyError {
    /// mint_rmb 以被创建
    #[error("Mint Account Exist")]
    MintRmbExist,
    #[error("Token Account Exist")]
    TokenRmbExist,
    #[error("Token Account Non-Exist")]
    TokenRmbNonExist,
}

impl From<MyError> for ProgramError {
    fn from(e: MyError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for MyError {
    fn type_of() -> &'static str {
        // 9527: 啥也不是,就是为了区分这是我程序的Error(也懒得给Error命名)
        "MyError(9527)"
    }
}

impl PrintProgramError for MyError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + solana_program::decode_error::DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            _ => msg!("Error: {}", self),
        }
    }
}
