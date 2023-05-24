pub mod entrypoint;
pub mod instruction;
pub mod state;
mod processor;
mod ecdsa;

const PDA_ADMIN_SEED: &str = "admin-ecdsa-verification-account";
