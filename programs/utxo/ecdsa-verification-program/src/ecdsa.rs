use solana_program::secp256k1_recover::{SECP256K1_PUBLIC_KEY_LENGTH, secp256k1_recover, SECP256K1_SIGNATURE_LENGTH};
use solana_program::{entrypoint::ProgramResult, msg};
use solana_program::program_error::ProgramError;
use utxo_base_program::error::UTXOError;

pub fn verify_ecdsa_signature(hash: &[u8], sig: &[u8], reid: u8, target_key: [u8; SECP256K1_PUBLIC_KEY_LENGTH]) -> ProgramResult {
    let recovered_key = secp256k1_recover(hash, reid, sig);
    if recovered_key.is_err() {
        return ProgramResult::Err(UTXOError::InvalidWitness.into());
    }

    let key =  recovered_key.unwrap().0;

    msg!("Recovered public key from signature: {}", bs58::encode(key.as_ref()).into_string().as_str());
    msg!("Required public key: {}", bs58::encode(target_key.as_ref()).into_string().as_str());

    if key != target_key {
        return ProgramResult::Err(UTXOError::InvalidWitness.into());
    }

    msg!("Public keys are equal");
    Ok(())
}

pub fn parse_witness(witness: &Vec<u8>) -> Result<(Vec<u8>, u8), ProgramError> {
    if witness.len() != SECP256K1_SIGNATURE_LENGTH + 1 {
        return Result::Err(UTXOError::InvalidWitness.into());
    }

    let (signature, reid) = witness.split_at(SECP256K1_SIGNATURE_LENGTH);
    return Result::Ok((Vec::from(signature), reid[0]))
}