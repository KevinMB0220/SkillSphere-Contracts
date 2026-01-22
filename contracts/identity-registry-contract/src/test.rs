#![cfg(test)]

extern crate std;

use crate::error::RegistryError;
use crate::types::ExpertStatus;
use crate::{IdentityRegistryContract, IdentityRegistryContractClient};
use soroban_sdk::testutils::{AuthorizedFunction, AuthorizedInvocation, Events};
use soroban_sdk::{testutils::Address as _, Address, Env, IntoVal, Symbol, TryIntoVal};

#[test]
fn test_initialization() {
    let env = Env::default();
    let contract_id = env.register(IdentityRegistryContract, ());
    let client = IdentityRegistryContractClient::new(&env, &contract_id);

    // 1. Generate a fake admin address
    let admin = soroban_sdk::Address::generate(&env);

    // 2. Call init (Should succeed)
    let res = client.try_init(&admin);
    assert!(res.is_ok());

    // 3. Call init again (Should fail)
    let res_duplicate = client.try_init(&admin);
    assert!(res_duplicate.is_err());
}

#[test]
fn test_add_expert() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(IdentityRegistryContract, ());
    let client = IdentityRegistryContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let expert = Address::generate(&env);

    client.init(&admin);

    let res = client.try_add_expert(&expert);
    assert!(res.is_ok());

    assert_eq!(
        env.auths()[0],
        (
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract_id.clone(),
                    Symbol::new(&env, "add_expert"),
                    (expert.clone(),).into_val(&env)
                )),
                sub_invocations: std::vec![]
            }
        )
    );
}

#[test]
#[should_panic]
fn test_add_expert_unauthorized() {
    let env = Env::default();

    let contract_id = env.register(IdentityRegistryContract, ());
    let client = IdentityRegistryContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let expert = Address::generate(&env);

    client.init(&admin);

    client.add_expert(&expert);
}

#[test]
fn test_expert_status_changed_event() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(IdentityRegistryContract, ());
    let client = IdentityRegistryContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let expert = Address::generate(&env);

    client.init(&admin);
    client.add_expert(&expert);

    let events = env.events().all();
    let event = events.last().unwrap();

    assert_eq!(event.0, contract_id);

    let topic: Symbol = event.1.get(0).unwrap().try_into_val(&env).unwrap();
    assert_eq!(topic, Symbol::new(&env, "status_change"));
}
#[test]
fn test_ban_expert() {
    let env = Env::default();
    let contract_id = env.register(IdentityRegistryContract, ());
    let client = IdentityRegistryContractClient::new(&env, &contract_id);

    // Setup: Create admin and expert addresses
    let admin = Address::generate(&env);
    let expert = Address::generate(&env);

    // Initialize the contract
    client.init(&admin);

    // Verify the expert first
    env.mock_all_auths();
    client.add_expert(&expert);

    // Verify status is Verified
    let status = client.get_status(&expert);
    assert_eq!(status, ExpertStatus::Verified);

    // Ban the expert (should succeed)
    client.ban_expert(&expert);

    // Check that status is now Banned
    let status = client.get_status(&expert);
    assert_eq!(status, ExpertStatus::Banned);

    // Test: Try to ban again (should fail with AlreadyBanned)
    let result = client.try_ban_expert(&expert);
    assert_eq!(result, Err(Ok(RegistryError::AlreadyBanned)));
}

#[test]
#[should_panic]
fn test_ban_expert_unauthorized() {
    let env = Env::default();
    let contract_id = env.register(IdentityRegistryContract, ());
    let client = IdentityRegistryContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let expert = Address::generate(&env);

    client.init(&admin);

    env.mock_all_auths();
    client.add_expert(&expert);

    env.mock_all_auths_allowing_non_root_auth();

    env.mock_auths(&[]);

    client.ban_expert(&expert);
}

#[test]
fn test_ban_unverified_expert() {
    let env = Env::default();
    let contract_id = env.register(IdentityRegistryContract, ());
    let client = IdentityRegistryContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let expert = Address::generate(&env);

    // Initialize
    client.init(&admin);

    // Verify initial status is Unverified
    let status = client.get_status(&expert);
    assert_eq!(status, ExpertStatus::Unverified);

    // Ban an expert who was never verified (should still succeed)
    env.mock_all_auths();
    client.ban_expert(&expert);

    // Status should be Banned now
    let status = client.get_status(&expert);
    assert_eq!(status, ExpertStatus::Banned);
}

#[test]
fn test_ban_expert_workflow() {
    let env = Env::default();
    let contract_id = env.register(IdentityRegistryContract, ());
    let client = IdentityRegistryContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let expert1 = Address::generate(&env);
    let expert2 = Address::generate(&env);
    let expert3 = Address::generate(&env);

    // Initialize
    client.init(&admin);

    env.mock_all_auths();

    // Verify multiple experts
    client.add_expert(&expert1);
    client.add_expert(&expert2);
    client.add_expert(&expert3);

    // Check all are verified
    assert_eq!(client.get_status(&expert1), ExpertStatus::Verified);
    assert_eq!(client.get_status(&expert2), ExpertStatus::Verified);
    assert_eq!(client.get_status(&expert3), ExpertStatus::Verified);

    // Ban expert2
    client.ban_expert(&expert2);

    // Verify expert2 is banned, others remain verified
    assert_eq!(client.get_status(&expert1), ExpertStatus::Verified);
    assert_eq!(client.get_status(&expert2), ExpertStatus::Banned);
    assert_eq!(client.get_status(&expert3), ExpertStatus::Verified);

    // Ban expert1
    client.ban_expert(&expert1);

    // Verify expert1 is now banned
    assert_eq!(client.get_status(&expert1), ExpertStatus::Banned);
    assert_eq!(client.get_status(&expert2), ExpertStatus::Banned);
    assert_eq!(client.get_status(&expert3), ExpertStatus::Verified);
}

#[test]
fn test_ban_before_contract_initialized() {
    let env = Env::default();
    let contract_id = env.register(IdentityRegistryContract, ());
    let client = IdentityRegistryContractClient::new(&env, &contract_id);

    let expert = Address::generate(&env);

    env.mock_all_auths();

    // Try to ban without initializing (should fail)
    let result = client.try_ban_expert(&expert);
    assert_eq!(result, Err(Ok(RegistryError::NotInitialized)));
}

#[test]
fn test_complete_expert_lifecycle() {
    let env = Env::default();
    let contract_id = env.register(IdentityRegistryContract, ());
    let client = IdentityRegistryContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let expert = Address::generate(&env);

    // Initialize
    client.init(&admin);

    env.mock_all_auths();

    // 1. Initial state: Unverified
    assert_eq!(client.get_status(&expert), ExpertStatus::Unverified);

    // 2. Verify the expert
    client.add_expert(&expert);
    assert_eq!(client.get_status(&expert), ExpertStatus::Verified);

    // 3. Ban the expert
    client.ban_expert(&expert);
    assert_eq!(client.get_status(&expert), ExpertStatus::Banned);
}
