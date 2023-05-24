use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct DepositSolArgs {
    pub amount: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct WithdrawSolArgs {
    pub witness: Vec<u8>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct VerifyTransferArgs {
    pub witness: Vec<Vec<u8>>,
    pub input_amount: usize,
    pub output_amount: usize,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum Instruction {
    /// Initialize new VerificationAdmin
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The VerificationAdmin account to initialize
    ///   1. `[writable,signer]` The fee payer
    ///   2. `[]` System program
    ///   3. `[]` Rent sysvar
    InitializeAdmin,

    /// Deposit Sol tokens and create UTXO
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The VerificationAdmin account
    ///   1. `[]` UTXO base program
    ///   2. `[writable, signer]` Payer
    ///   3. `[writable]` UTXO to deposit (should be already initialized)
    DepositSol(DepositSolArgs),

    /// Withdraw Sol tokens
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The VerificationAdmin account
    ///   1. `[]` UTXO base program
    ///   2. `[writable]` Receiver
    ///   3. `[writable]` UTXO to withdraw
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