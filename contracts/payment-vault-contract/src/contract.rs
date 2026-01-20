use soroban_sdk::{Address, Env};
use crate::storage;
use crate::error::VaultError;

pub fn initialize_vault(
    env: &Env, 
    admin: &Address, 
    token: &Address, 
    oracle: &Address
) -> Result<(), VaultError> {
    // 1. Check if already initialized
    if storage::has_admin(env) {
        return Err(VaultError::AlreadyInitialized);
    }

    // 2. Save State
    storage::set_admin(env, admin);
    storage::set_token(env, token);
    storage::set_oracle(env, oracle);

    Ok(())
}