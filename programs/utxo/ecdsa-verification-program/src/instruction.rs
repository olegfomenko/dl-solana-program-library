use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct DepositSolArgs {
    pub amount: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct WithdrawSolArgs {
    pub witness: Vec<Vec<u8>>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct VerifyTransferArgs {
    pub witness: Vec<Vec<u8>>,
    pub input_amount: u8,
    pub output_amount: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum Instruction {
    /// Deposit Sol tokens and create UTXO
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[]` UTXO base program
    ///   1. `[writable, signer]` Payer
    ///   2. `[writable]` UTXO to deposit (currently uninitialized)
    DepositSol(DepositSolArgs),

    /// Withdraw Sol tokens
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[]` UTXO base program
    ///   1. `[writable]` Receiver
    ///   2. `[writable]` UTXO to withdraw
    WithdrawSol(WithdrawSolArgs),

    /// Execute transfer from one UTXO to another
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[]` UTXO base program
    ///   {1...input_amount}. `[writable]` UTXOs from
    ///   {input_amount+1...input_amount+output_amount}. `[writable]` UTXOs to
    Transfer(VerifyTransferArgs),
}