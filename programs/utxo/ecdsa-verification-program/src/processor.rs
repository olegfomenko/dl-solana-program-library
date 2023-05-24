use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, msg,
    program::{invoke_signed, invoke}, pubkey::Pubkey, system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use borsh::{BorshDeserialize, BorshSerialize};
use crate::instruction::Instruction;


pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    input: &[u8],
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(input)?;
    match instruction {
        Instruction::DepositSol(args) => {
            msg!("Instruction: Deposit Sol");
            Ok(())
        }
        Instruction::WithdrawSol(args) => {
            msg!("Instruction: Withdraw Sol");
            Ok(())
        }
        Instruction::Transfer(args) => {
            msg!("Instruction: Transfer UTXO");
            Ok(())
        }
    }
}
