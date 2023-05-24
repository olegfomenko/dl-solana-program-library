use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, msg,
    program::{invoke_signed, invoke}, pubkey::Pubkey, system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use borsh::{BorshDeserialize, BorshSerialize};
use crate::instruction::Instruction;
use crate::PDA_ADMIN_SEED;
use utxo_base_program::error::UTXOError;
use crate::state::{MAX_ADMIN_SIZE, VerificationAdmin};
use utxo_base_program::instruction::{initialize_utxo, activate_utxo, deactivate_utxo};
use solana_program::secp256k1_recover::SECP256K1_PUBLIC_KEY_LENGTH;
use utxo_base_program::state::UTXO;
use std::mem::size_of;
use crate::ecdsa::{verify_ecdsa_signature, parse_witness};
use std::convert::TryInto;


pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    input: &[u8],
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(input)?;
    match instruction {
        Instruction::InitializeAdmin => {
            msg!("Instruction: Initialize admin");
            process_init_admin(program_id, accounts)
        }
        Instruction::DepositSol(args) => {
            msg!("Instruction: Deposit Sol");
            process_deposit_sol(program_id, accounts, args.amount)
        }
        Instruction::WithdrawSol(args) => {
            msg!("Instruction: Withdraw Sol");
            process_withdraw_sol(program_id, accounts, args.witness)
        }
        Instruction::Transfer(args) => {
            msg!("Instruction: Transfer UTXO");
            process_transfer(program_id, accounts, args.witness, args.input_amount, args.output_amount)
        }
    }
}

pub fn process_init_admin<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let admin_info = next_account_info(account_info_iter)?;
    let fee_payer_info = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    let (admin_key, bump) = Pubkey::find_program_address(&[PDA_ADMIN_SEED.as_bytes()], &program_id);
    if admin_key != *admin_info.key {
        return Err(UTXOError::WrongAdmin.into());
    }

    let rent = Rent::from_account_info(rent_info)?;

    let instruction = system_instruction::create_account(
        fee_payer_info.key,
        admin_info.key,
        rent.minimum_balance(MAX_ADMIN_SIZE),
        MAX_ADMIN_SIZE as u64,
        program_id,
    );

    invoke_signed(
        &instruction,
        &[
            fee_payer_info.clone(),
            admin_info.clone(),
            system_program.clone(),
        ],
        &[&[PDA_ADMIN_SEED.as_bytes(), &[bump]]],
    )?;

    let mut admin: VerificationAdmin = BorshDeserialize::deserialize(&mut admin_info.data.borrow_mut().as_ref())?;
    if admin.is_initialized {
        return Err(UTXOError::AlreadyInUse.into());
    }

    admin.is_initialized = true;
    admin.serialize(&mut *admin_info.data.borrow_mut())?;
    Ok(())
}

pub fn process_deposit_sol<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let admin_info = next_account_info(account_info_iter)?;
    let base_program = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let utxo_info = next_account_info(account_info_iter)?;

    let mut utxo: UTXO = BorshDeserialize::deserialize(&mut utxo_info.data.borrow_mut().as_ref())?;
    if !utxo.is_initialized {
        return Err(UTXOError::NotInitialized.into());
    }

    let (utxo_key, bump) = Pubkey::find_program_address(&[utxo.account_seed.as_slice()], program_id);
    if utxo_key != *utxo_info.key {
        return Err(UTXOError::WrongSeed.into());
    }

    verify_utxo_data(&utxo.verification_data, &utxo.content_data, amount)?;

    let activate_utxo_instruction = activate_utxo(
        *base_program.key,
        program_id,
        utxo.account_seed,
    );

    msg!("Activating UTXO");
    invoke_signed(
        &activate_utxo_instruction,
        &[
            utxo_info.clone(),
        ],
        &[&[utxo.account_seed.as_slice(), &[bump]]],
    )?;


    let transfer_tokens_instruction = solana_program::system_instruction::transfer(
        payer_info.key,
        admin_info.key,
        amount,
    );

    msg!("Transferring Sol");
    invoke(
        &transfer_tokens_instruction,
        &[
            payer_info.clone(),
            admin_info.clone(),
        ],
    )?;

    Ok(())
}


pub fn process_withdraw_sol<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    witness: Vec<u8>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let admin_info = next_account_info(account_info_iter)?;
    let base_program = next_account_info(account_info_iter)?;
    let receiver_info = next_account_info(account_info_iter)?;
    let utxo_info = next_account_info(account_info_iter)?;

    let mut utxo: UTXO = BorshDeserialize::deserialize(&mut utxo_info.data.borrow_mut().as_ref())?;
    if !utxo.is_initialized {
        return Err(UTXOError::NotInitialized.into());
    }

    let (utxo_key, bump) = Pubkey::find_program_address(&[utxo.account_seed.as_slice()], program_id);
    if utxo_key != *utxo_info.key {
        return Err(UTXOError::WrongSeed.into());
    }

    let amount_array: [u8; size_of::<u64>()] = utxo.content_data.as_slice().try_into().expect("invalid size");
    let amount = u64::from_be_bytes(amount_array);


    let (signature, reid) = parse_witness(&witness)?;
    let public_key: [u8; SECP256K1_PUBLIC_KEY_LENGTH] = utxo.verification_data.as_slice().try_into().expect("invalid size");

    verify_ecdsa_signature(
        solana_program::keccak::hash(
            utxo_key.as_ref()
        ).as_ref(),
        signature.as_slice(),
        reid,
        public_key,
    )?;

    let deactivate_utxo_instruction = deactivate_utxo(
        *base_program.key,
        program_id,
        utxo.account_seed,
    );

    msg!("Deactivating UTXO");
    invoke_signed(
        &deactivate_utxo_instruction,
        &[
            utxo_info.clone(),
        ],
        &[&[utxo.account_seed.as_slice(), &[bump]]],
    )?;


    let transfer_tokens_instruction = solana_program::system_instruction::transfer(
        admin_info.key,
        receiver_info.key,
        amount,
    );

    msg!("Transferring Sol");
    invoke(
        &transfer_tokens_instruction,
        &[
            receiver_info.clone(),
            admin_info.clone(),
        ],
    )?;

    Ok(())
}


pub fn process_transfer<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    witness: Vec<Vec<u8>>,
    inputs_amount: usize,
    outputs_amount: usize,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let base_program = next_account_info(account_info_iter)?;

    if witness.len() != inputs_amount {
        return Err(UTXOError::InvalidTransferData.into());
    }

    let mut inputs: Vec<&AccountInfo> = Vec::new();
    let mut outputs: Vec<&AccountInfo> = Vec::new();

    let mut hash_keys: Vec<&[u8]> = Vec::new();
    hash_keys.push(&[]);

    let mut input_sum: u64 = 0;
    let mut output_sum: u64 = 0;

    for i in 0..outputs_amount {
        let output = next_account_info(account_info_iter)?;
        outputs.push(output);
        hash_keys.push(output.key.as_ref());

        let utxo: UTXO = BorshDeserialize::deserialize(&mut output.data.borrow_mut().as_ref())?;
        if !utxo.is_initialized {
            return Err(UTXOError::NotInitialized.into());
        }

        let (utxo_key, bump) = Pubkey::find_program_address(&[utxo.account_seed.as_slice()], program_id);
        if utxo_key != *output.key {
            return Err(UTXOError::WrongSeed.into());
        }

        let amount_array: [u8; size_of::<u64>()] = utxo.content_data.as_slice().try_into().expect("invalid size");
        let amount = u64::from_be_bytes(amount_array);
        output_sum += amount;

        let activate_utxo_instruction = activate_utxo(
            *base_program.key,
            program_id,
            utxo.account_seed,
        );

        msg!("Activating UTXO {}", i);
        invoke_signed(
            &activate_utxo_instruction,
            &[
                output.clone(),
            ],
            &[&[utxo.account_seed.as_slice(), &[bump]]],
        )?;
    }

    for i in 0..inputs_amount {
        let input = next_account_info(account_info_iter)?;
        inputs.push(input);

        let utxo: UTXO = BorshDeserialize::deserialize(&mut input.data.borrow_mut().as_ref())?;
        if !utxo.is_initialized {
            return Err(UTXOError::NotInitialized.into());
        }

        let (utxo_key, bump) = Pubkey::find_program_address(&[utxo.account_seed.as_slice()], program_id);
        if utxo_key != *input.key {
            return Err(UTXOError::WrongSeed.into());
        }

        let amount_array: [u8; size_of::<u64>()] = utxo.content_data.as_slice().try_into().expect("invalid size");
        let amount = u64::from_be_bytes(amount_array);
        input_sum += amount;

        hash_keys[0] = input.key.as_ref();

        let (signature, reid) = parse_witness(witness.get(i).unwrap())?;
        let public_key: [u8; SECP256K1_PUBLIC_KEY_LENGTH] = utxo.verification_data.as_slice().try_into().expect("invalid size");

        verify_ecdsa_signature(
            solana_program::keccak::hash(
                &hash_keys.as_slice().concat()
            ).as_ref(),
            signature.as_slice(),
            reid,
            public_key,
        )?;

        let deactivate_utxo_instruction = deactivate_utxo(
            *base_program.key,
            program_id,
            utxo.account_seed,
        );

        msg!("Deactivating UTXO {}", i);
        invoke_signed(
            &deactivate_utxo_instruction,
            &[
                input.clone(),
            ],
            &[&[utxo.account_seed.as_slice(), &[bump]]],
        )?;
    }

    if input_sum != output_sum {
        return Err(UTXOError::InvalidTransferData.into());
    }

    Ok(())
}

pub fn verify_utxo_data(verification_data: &Vec<u8>, content_data: &Vec<u8>, amount: u64) -> ProgramResult {
    if verification_data.len() != SECP256K1_PUBLIC_KEY_LENGTH {
        return Err(UTXOError::InvalidData.into());
    }

    if content_data.ne(&Vec::from(amount.to_be_bytes())) {
        return Err(UTXOError::InvalidData.into());
    }

    Ok(())
}
