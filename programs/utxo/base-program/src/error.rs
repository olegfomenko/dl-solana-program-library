//! Error types

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError}};
use thiserror::Error;

/// Errors that may be returned by the Token program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum UTXOError {
    /// 0 The account cannot be initialized because it is already being used.
    #[error("Already in use")]
    AlreadyInUse,
    /// 1 The account hasn't been initialized
    #[error("Not initialized")]
    NotInitialized,
    /// 2 Wrong seed provided
    #[error("Wrong seed")]
    WrongSeed,
    /// 3 Account should be signer
    #[error("Unsigned")]
    Unsigned,
    /// 4 Wrong admin account
    #[error("Wrong admin")]
    WrongAdmin,
    /// 5 UTXO already activated
    #[error("UTXO already activated")]
    AlreadyActivated,
    /// 6 Invalid UTXO data
    #[error("Invalid UTXO data")]
    InvalidData,
    /// 7 Invalid witness
    #[error("Invalid witness")]
    InvalidWitness,
}


impl From<UTXOError> for ProgramError {
    fn from(e: UTXOError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl PrintProgramError for UTXOError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl<T> DecodeError<T> for UTXOError {
    fn type_of() -> &'static str {
        "UTXOError"
    }
}
