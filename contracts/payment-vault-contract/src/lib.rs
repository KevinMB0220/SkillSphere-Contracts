#![no_std]

mod contract;
mod error;
mod storage;
#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Address, Env};
use crate::error::VaultError;

#[contract]
pub struct PaymentVaultContract;

#[contractimpl]
impl PaymentVaultContract {
    /// Initialize the vault with the Admin, the Payment Token, and the Oracle (Backend)
    pub fn init(
        env: Env, 
        admin: Address, 
        token: Address, 
        oracle: Address
    ) -> Result<(), VaultError> {
        contract::initialize_vault(&env, &admin, &token, &oracle)
    }
}