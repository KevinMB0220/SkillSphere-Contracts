use crate::types::ExpertStatus;
use soroban_sdk::{contracttype, Address, Env, Symbol, String};

// The Event Data Structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpertStatusChangedEvent {
    pub expert: Address,
    pub old_status: ExpertStatus,
    pub new_status: ExpertStatus,
    pub admin: Address,
}

// Helper function to emit the status change event
#[allow(deprecated)]
pub fn emit_status_change(
    env: &Env,
    expert: Address,
    old_status: ExpertStatus,
    new_status: ExpertStatus,
    admin: Address,
) {
    let event = ExpertStatusChangedEvent {
        expert,
        old_status,
        new_status,
        admin,
    };

    // published with the topic "status_change" so indexers can find it easily
    env.events()
        .publish((Symbol::new(env, "status_change"),), event);
}

// Event for profile URI updates
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileUpdatedEvent {
    pub expert: Address,
    pub new_uri: String,
}

#[allow(deprecated)]
pub fn emit_profile_updated(env: &Env, expert: Address, new_uri: String) {
    let event = ProfileUpdatedEvent { expert, new_uri };
    env.events()
        .publish((Symbol::new(env, "profile_updated"),), event);
}
