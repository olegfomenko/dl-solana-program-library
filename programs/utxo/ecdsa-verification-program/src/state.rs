use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub const MAX_ADMIN_SIZE: usize =  (8 as usize) + (1 as usize);

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct VerificationAdmin {
    pub total_locked: u64,
    pub is_initialized: bool,
}
