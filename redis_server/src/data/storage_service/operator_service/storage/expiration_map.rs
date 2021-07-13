use std::collections::HashMap;

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

    pub fn contains(&self, key: &str) {
        assert_eq!(self.map.len(), self.vec.len());
        self.map.contains_key(key);
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
