use crate::utilities::current_time_in_millis;
use std::collections::HashMap;

use rand::seq::IteratorRandom;
use rand::thread_rng;

#[derive(Debug, Default)]
pub struct ExpirationMap {
    map: HashMap<String, u128>,
}

impl ExpirationMap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn expire_at(&mut self, key: String, timestamp: u128) -> Option<u128> {
        self.map.insert(key, timestamp)
    }

    pub fn expire_in(&mut self, key: String, ms: u128) -> Option<u128> {
        let timestamp = current_time_in_millis() + ms;
        self.map.insert(key, timestamp)
    }

    pub fn is_expirable(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn is_expired(&self, key: &str) -> bool {
        match self.map.get(key) {
            None => false,
            Some(value) => current_time_in_millis() > *value,
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<u128> {
        self.map.remove(key)
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn get(&self, key: &str) -> Option<u128> {
        self.map.get(key).cloned()
    }

    pub fn get_random_volatile_keys(&self, amount: usize) -> Vec<String> {
        let mut rng = thread_rng();
        self.map
            .keys()
            .choose_multiple(&mut rng, amount)
            .into_iter()
            .cloned()
            .collect()
    }
}
