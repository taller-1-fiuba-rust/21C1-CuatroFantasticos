use std::collections::HashMap;

use value::StorageValue;

use crate::data::redis_value::list::RedisValueList;
use crate::data::redis_value::set::RedisValueSet;
use crate::data::redis_value::string::RedisValueString;
use crate::data::redis_value::RedisValue;
use crate::data::storage_service::operator_service::storage::expiration_map::ExpirationMap;
use std::fmt::Debug;

pub mod expiration_map;
pub mod value;

#[derive(Debug, Default)]
pub struct RedisStorage {
    values: HashMap<String, StorageValue>,
    expirations: ExpirationMap,
}

impl RedisStorage {
    pub fn new() -> Self {
        Default::default()
    }

    fn clean_if_expirated(&mut self, key: &str) {
        if self.values.contains_key(key) && self.expirations.is_expired(key) {
            let _ = self.values.remove(key);
            let _ = self.expirations.remove(key);
        }
    }

    pub fn insert(&mut self, key: &str, value: RedisValue) -> Option<RedisValue> {
        self.clean_if_expirated(key);
        let storage_value = StorageValue::new(value);
        let old_value = self.values.insert(key.to_string(), storage_value);
        let _ = self.expirations.remove(key);
        old_value.map(|v| v.extract_value())
    }

    pub fn update(&mut self, key: &str, value: RedisValue) -> Option<RedisValue> {
        self.clean_if_expirated(key);
        if !self.values.contains_key(key) {
            return None;
        }
        let storage_value = StorageValue::new(value);
        let old_value = self.values.insert(key.to_string(), storage_value);
        old_value.map(|v| v.extract_value())
    }

    pub fn access(&mut self, key: &str) -> Option<&RedisValue> {
        self.clean_if_expirated(key);
        let storage_value = self.values.get_mut(key);
        storage_value.map(|v| v.access())
    }

    pub fn length(&self) -> usize {
        self.values.len()
    }

    pub fn get(&mut self, key: &str) -> Option<&RedisValue> {
        self.clean_if_expirated(key);
        self.values.get_mut(key).map(|value| value.access())
    }

    pub fn mut_get(&mut self, key: &str) -> Option<&mut RedisValue> {
        self.clean_if_expirated(key);
        self.values.get_mut(key).map(|value| value.access_mut())
    }

    pub fn contains_key(&mut self, key: &str) -> bool {
        self.clean_if_expirated(key);
        self.values.contains_key(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<RedisValue> {
        self.clean_if_expirated(key);
        self.values.remove(key).map(|v| v.extract_value())
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.expirations.clear();
    }

    pub fn serialize(&self) -> Vec<String> {
        todo!()
        /*
        let mut contents = Vec::new();
        for (key, value) in &self.values {
            let value = value.peek();
            match value {
                Some(value) => {
                    let line = format!("{}: {}", key, value.serialize());
                    contents.push(line);
                }
                None => continue,
            }
        }
        contents
         */
    }

    pub fn deserialize(contents: String) -> RedisStorage {
        let mut storage = RedisStorage::new();
        for line in contents.lines() {
            let split = line.split(':');
            let parsed_line: Vec<&str> = split.collect();
            match parsed_line[1].trim() {
                "string" => {
                    let value = RedisValueString::new(parsed_line[2].trim().to_owned());
                    storage.insert(parsed_line[0].trim(), RedisValue::String(value));
                }
                "list" => {
                    let value = RedisValueList::new(parsed_line[2].trim().to_owned());
                    storage.insert(parsed_line[0].trim(), RedisValue::List(value));
                }
                "set" => {
                    let value = RedisValueSet::new(parsed_line[2].trim().to_owned());
                    storage.insert(parsed_line[0].trim(), RedisValue::Set(value));
                }
                _ => println!("Data type not supported in deserialization"),
            }
        }
        storage
    }
}
