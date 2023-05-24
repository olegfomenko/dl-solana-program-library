use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct UTXO {
    pub base_program: Pubkey,
    pub verification_program: Pubkey,
    pub verification_data: Vec<u8>,
    pub content_data: Vec<u8>,
    pub is_active: bool,
    pub account_seed: [u8; 32],
    pub is_initialized: bool,
}

pub fn get_utxo_size(verification_data_len:usize, content_data_len:usize) -> usize {
    (96 as usize) + verification_data_len + content_data_len + (2 as usize)
}