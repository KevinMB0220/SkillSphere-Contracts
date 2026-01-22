use crate::error::RegistryError;
use crate::events;
use crate::storage;
use crate::types::ExpertStatus;
use soroban_sdk::{Address, Env};

pub fn initialize_registry(env: &Env, admin: &Address) -> Result<(), RegistryError> {
    if storage::has_admin(env) {
        return Err(RegistryError::AlreadyInitialized);
    }

    storage::set_admin(env, admin);

    Ok(())
}

pub fn verify_expert(env: &Env, expert: &Address) -> Result<(), RegistryError> {
    let admin = storage::get_admin(env).ok_or(RegistryError::NotInitialized)?;

    admin.require_auth();

    let current_status = storage::get_expert_status(env, expert);

    if current_status == ExpertStatus::Verified {
        return Err(RegistryError::AlreadyVerified);
    }

    storage::set_expert_record(env, expert, ExpertStatus::Verified);

    events::emit_status_change(
        env,
        expert.clone(),
        current_status,
        ExpertStatus::Verified,
        admin,
    );

    Ok(())
}

pub fn ban_expert(env: &Env, expert: &Address) -> Result<(), RegistryError> {
    let admin = storage::get_admin(env).ok_or(RegistryError::NotInitialized)?;
    admin.require_auth();

    let current_status = storage::get_expert_status(env, expert);

    if current_status == ExpertStatus::Banned {
        return Err(RegistryError::AlreadyBanned);
    }

    storage::set_expert_record(env, expert, ExpertStatus::Banned);

    events::emit_status_change(
        env,
        expert.clone(),
        current_status,
        ExpertStatus::Banned,
        admin,
    );

    Ok(())
}

pub fn get_expert_status(env: &Env, expert: &Address) -> ExpertStatus {
    storage::get_expert_status(env, expert)
}
