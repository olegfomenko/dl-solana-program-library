use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, msg,
    program::{invoke_signed, invoke}, pubkey::Pubkey, system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use crate::state::{UTXO, get_utxo_size};
use borsh::{BorshDeserialize, BorshSerialize};
use crate::error::UTXOError;
use crate::instruction::Instruction;


pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    input: &[u8],
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(input)?;
    match instruction {
        Instruction::InitializeUTXO(args) => {
            msg!("Instruction: Initialize UTXO");
            process_init_utxo(program_id, accounts, args.verification_program, args.verification_data, args.content_data, args.account_seed)
        }
        Instruction::ActivateUTXO => {
            msg!("Instruction: Activate UTXO");
            process_activate(program_id, accounts)
        }
        Instruction::DeactivateUTXO => {
            msg!("Instruction: Deactivate UTXO");
            process_deactivate(program_id, accounts)
        }
    }
}

pub fn process_init_utxo<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    verification_program: Pubkey,
    verification_data: Vec<u8>,
    content_data: Vec<u8>,
    account_seed: [u8; 32],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let utxo_info = next_account_info(account_info_iter)?;
    let fee_payer_info = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    let (utxo_key, bump) = Pubkey::find_program_address(&[account_seed.as_slice()], &verification_program);
    if utxo_key != *utxo_info.key {
        return Err(UTXOError::WrongSeed.into());
    }

    let rent = Rent::from_account_info(rent_info)?;

    let size = get_utxo_size(verification_data.len(), content_data.len());
    let instruction = system_instruction::create_account(
        fee_payer_info.key,
        utxo_info.key,
        rent.minimum_balance(size),
        size as u64,
        program_id,
    );

    invoke(
        &instruction,
        &[
            fee_payer_info.clone(),
            utxo_info.clone(),
            system_program.clone(),
        ],
    )?;

    let mut utxo: UTXO = BorshDeserialize::deserialize(&mut utxo_info.data.borrow_mut().as_ref())?;
    if utxo.is_initialized {
        return Err(UTXOError::AlreadyInUse.into());
    }

    utxo.content_data = content_data;
    utxo.verification_data = verification_data;
    utxo.verification_program = verification_program;
    utxo.base_program = *program_id;
    utxo.account_seed = account_seed;
    utxo.is_initialized = true;
    utxo.serialize(&mut *utxo_info.data.borrow_mut())?;
    Ok(())
}


pub fn process_activate<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let utxo_info = next_account_info(account_info_iter)?;

    let mut utxo: UTXO = BorshDeserialize::deserialize(&mut utxo_info.data.borrow_mut().as_ref())?;
    if !utxo.is_initialized {
        return Err(UTXOError::NotInitialized.into());
    }

    let (utxo_key, _) = Pubkey::find_program_address(&[utxo.account_seed.as_slice()], &utxo.verification_program);
    if utxo_key != *utxo_info.key {
        return Err(UTXOError::WrongSeed.into());
    }

    if !utxo_info.is_signer {
        return Err(UTXOError::Unsigned.into());
    }

    utxo.is_active = true;
    utxo.serialize(&mut *utxo_info.data.borrow_mut())?;
    Ok(())
}

pub fn process_deactivate<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let utxo_info = next_account_info(account_info_iter)?;

    let mut utxo: UTXO = BorshDeserialize::deserialize(&mut utxo_info.data.borrow_mut().as_ref())?;
    if !utxo.is_initialized {
        return Err(UTXOError::NotInitialized.into());
    }

    let (utxo_key, _) = Pubkey::find_program_address(&[utxo.account_seed.as_slice()], &utxo.verification_program);
    if utxo_key != *utxo_info.key {
        return Err(UTXOError::WrongSeed.into());
    }

    if !utxo_info.is_signer {
        return Err(UTXOError::Unsigned.into());
    }

    utxo.is_active = false;
    utxo.serialize(&mut *utxo_info.data.borrow_mut())?;
    Ok(())
}