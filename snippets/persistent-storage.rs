#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String, Symbol, Vec};

/// One key per value we want to keep around. Using an enum keeps
/// all keys in one place and lets the compiler catch typos.
#[contracttype]
pub enum DataKey {
    Counter,          // u32
    Greeting,         // String
    Owner,            // Address
    Scores,           // Vec<u32>
    Balances,         // Map<Symbol, i128>
    Profile(Address), // Profile, keyed per user
}

/// Structs marked with `#[contracttype]` can be stored directly.
#[contracttype]
#[derive(Clone)]
pub struct Profile {
    pub name: String,
    pub level: u32,
    pub active: bool,
}

// Persistent entries expire unless their TTL is extended.
// If the TTL drops below this many ledgers...
const TTL_THRESHOLD: u32 = 1000;
// ...bump it back up to this many.
const TTL_EXTEND_TO: u32 = 5000;

#[contract]
pub struct PersistentStorage;

#[contractimpl]
impl PersistentStorage {
    // ---------- Primitives ----------

    pub fn increment(env: Env) -> u32 {
        let store = env.storage().persistent();
        let count: u32 = store.get(&DataKey::Counter).unwrap_or(0);
        let count = count + 1;
        store.set(&DataKey::Counter, &count);
        store.extend_ttl(&DataKey::Counter, TTL_THRESHOLD, TTL_EXTEND_TO);
        count
    }

    pub fn set_greeting(env: Env, greeting: String) {
        let store = env.storage().persistent();
        store.set(&DataKey::Greeting, &greeting);
        store.extend_ttl(&DataKey::Greeting, TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    pub fn get_greeting(env: Env) -> Option<String> {
        env.storage().persistent().get(&DataKey::Greeting)
    }

    // ---------- Address ----------

    pub fn set_owner(env: Env, owner: Address) {
        owner.require_auth();
        let store = env.storage().persistent();
        store.set(&DataKey::Owner, &owner);
        store.extend_ttl(&DataKey::Owner, TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    pub fn get_owner(env: Env) -> Option<Address> {
        env.storage().persistent().get(&DataKey::Owner)
    }

    // ---------- Vec ----------

    /// Read the whole Vec, mutate it, write it back.
    pub fn add_score(env: Env, score: u32) -> Vec<u32> {
        let store = env.storage().persistent();
        let mut scores: Vec<u32> = store
            .get(&DataKey::Scores)
            .unwrap_or(Vec::new(&env));
        scores.push_back(score);
        store.set(&DataKey::Scores, &scores);
        store.extend_ttl(&DataKey::Scores, TTL_THRESHOLD, TTL_EXTEND_TO);
        scores
    }

    // ---------- Map ----------

    /// Maps behave the same way: load, update, store.
    pub fn credit(env: Env, asset: Symbol, amount: i128) -> i128 {
        let store = env.storage().persistent();
        let mut balances: Map<Symbol, i128> = store
            .get(&DataKey::Balances)
            .unwrap_or(Map::new(&env));
        let balance = balances.get(asset.clone()).unwrap_or(0) + amount;
        balances.set(asset, balance);
        store.set(&DataKey::Balances, &balances);
        store.extend_ttl(&DataKey::Balances, TTL_THRESHOLD, TTL_EXTEND_TO);
        balance
    }

    pub fn balance_of(env: Env, asset: Symbol) -> i128 {
        let balances: Map<Symbol, i128> = env
            .storage()
            .persistent()
            .get(&DataKey::Balances)
            .unwrap_or(Map::new(&env));
        balances.get(asset).unwrap_or(0)
    }

    // ---------- Struct, keyed per user ----------

    pub fn set_profile(env: Env, user: Address, name: String, level: u32) {
        user.require_auth();
        let key = DataKey::Profile(user);
        let profile = Profile {
            name,
            level,
            active: true,
        };
        let store = env.storage().persistent();
        store.set(&key, &profile);
        store.extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    pub fn get_profile(env: Env, user: Address) -> Option<Profile> {
        env.storage().persistent().get(&DataKey::Profile(user))
    }

    // ---------- Existence and removal ----------

    pub fn has_profile(env: Env, user: Address) -> bool {
        env.storage().persistent().has(&DataKey::Profile(user))
    }

    pub fn remove_profile(env: Env, user: Address) {
        user.require_auth();
        env.storage().persistent().remove(&DataKey::Profile(user));
    }
}
