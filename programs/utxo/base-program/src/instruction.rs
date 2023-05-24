use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct InitializeUTXOArgs {
    pub verification_program: Pubkey,
    pub verification_data: Vec<u8>,
    pub content_data: Vec<u8>,
    pub account_seed: [u8; 32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum Instruction {
    /// Initialize UTXO instruction. Should be calle only from verification program.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable,signer]` The UTXO account to initialize
    ///   1. `[writable,signer]` The fee payer
    ///   2. `[]` System program
    ///   3. `[]` Rent sysvar
    InitializeUTXO(InitializeUTXOArgs),

    /// Make UTXO active instruction. Should be calle only from verification program.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable,signer]` The UTXO account to activate
    ActivateUTXO,

    /// Make UTXO inactive instruction. Should be calle only from verification program.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable,signer]` The UTXO account to deactivate
    DeactivateUTXO,
}