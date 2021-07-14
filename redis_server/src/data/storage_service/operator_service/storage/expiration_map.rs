use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ExpirationMap {
    map: HashMap<String, u128>,
    vec: Vec<String>,
}

impl ExpirationMap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, key: String, value: u128) {
        assert_eq!(self.map.len(), self.vec.len());
        let old_value = self.map.insert(key.clone(), value);
        if old_value.is_none() {
            self.vec.push(key);
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        assert_eq!(self.map.len(), self.vec.len());
        self.map.contains_key(key)
    }

    pub fn is_expired(&self, key: &str) -> bool {
        match self.map.get(key){
            None => {false}
            Some(value) => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                now > *value
            }
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.vec.clear();
    }
}

impl Default for ExpirationMap {
    fn default() -> Self {
        ExpirationMap {
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }
}
