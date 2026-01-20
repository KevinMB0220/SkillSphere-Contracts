#![cfg(test)]
use crate::{PaymentVaultContract, PaymentVaultContractClient};
use soroban_sdk::{Env, testutils::Address as _};

fn create_client<'a>(env: &'a Env) -> PaymentVaultContractClient<'a> {
    let contract_id = env.register_contract(None, PaymentVaultContract);
    PaymentVaultContractClient::new(env, &contract_id)
}

#[test]
fn test_initialization() {
    let env = Env::default();
    let client = create_client(&env);

    let admin = soroban_sdk::Address::generate(&env);
    let token = soroban_sdk::Address::generate(&env); // Mock token address
    let oracle = soroban_sdk::Address::generate(&env); // Mock oracle address

    // 1. Successful Init
    let res = client.try_init(&admin, &token, &oracle);
    assert!(res.is_ok());

    // 2. Double Init (Should Fail)
    let res_duplicate = client.try_init(&admin, &token, &oracle);
    assert!(res_duplicate.is_err());

}