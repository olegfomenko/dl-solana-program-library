use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction::AccountMeta;

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
    ///   0. `[writable]` The UTXO account to initialize
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

pub fn initialize_utxo(
    program_id: Pubkey,
    fee_payer: Pubkey,
    account_seed: [u8; 32],
    verification_program: Pubkey,
    verification_data: Vec<u8>,
    content_data: Vec<u8>,
) -> solana_program::instruction::Instruction {
    let (utxo_key, _) = Pubkey::find_program_address(&[account_seed.as_slice()], &verification_program);

    solana_program::instruction::Instruction{
        program_id,
        data: Instruction::InitializeUTXO(
            InitializeUTXOArgs {
                verification_program,
                verification_data,
                content_data,
                account_seed
            }
        ).try_to_vec().unwrap(),
        accounts: vec![
            AccountMeta::new(utxo_key, false),
            AccountMeta::new(fee_payer, true),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ],
    }
}

pub fn activate_utxo(
    program_id: Pubkey,
    verification_program: &Pubkey,
    account_seed: [u8; 32],
) -> solana_program::instruction::Instruction {
    let (utxo_key, _) = Pubkey::find_program_address(&[account_seed.as_slice()], &verification_program);

    solana_program::instruction::Instruction{
        program_id,
        data: Instruction::ActivateUTXO.try_to_vec().unwrap(),
        accounts: vec![
            AccountMeta::new(utxo_key, true),
        ],
    }
}

pub fn deactivate_utxo(
    program_id: Pubkey,
    verification_program: &Pubkey,
    account_seed: [u8; 32],
) -> solana_program::instruction::Instruction {
    let (utxo_key, _) = Pubkey::find_program_address(&[account_seed.as_slice()], &verification_program);

    solana_program::instruction::Instruction{
        program_id,
        data: Instruction::DeactivateUTXO.try_to_vec().unwrap(),
        accounts: vec![
            AccountMeta::new(utxo_key, true),
        ],
    }
}