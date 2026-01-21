#![no_std]

mod contract;
mod error;
mod events;
mod storage;
mod types;
#[cfg(test)]
mod test;

use core::ops::Add;

use soroban_sdk::{contract, contractimpl, Address, Env, Vec};
use crate::{error::RegistryError, types::ExpertStatus};

#[contract]
pub struct IdentityRegistryContract;

#[contractimpl]
impl IdentityRegistryContract {
    /// Initialize the contract with an admin
    pub fn init(env: Env, admin: Address) -> Result<(), RegistryError> {
        contract::initialize_registry(&env, &admin)
    }

    /// Batch Verification
    pub fn batch_add_experts(env:Env, experts: Vec<Address>) -> Result<(), RegistryError> {
        if experts.len() > 20 {
            return Err(RegistryError::ExpertVecMax);
        }

        let admin = storage::get_admin(&env).unwrap();
        admin.require_auth();

        for expert in experts {
            let status = storage::get_expert_status(&env, &expert);
            if status == ExpertStatus::Verified {
                return Err(RegistryError::AlreadyVerified);
            }
            storage::set_expert_record(&env, &expert, ExpertStatus::Verified);
            events::emit_status_change(&env, expert, ExpertStatus::Unverified, ExpertStatus::Verified, admin.clone());
        }
        
        Ok(())
    }
} 