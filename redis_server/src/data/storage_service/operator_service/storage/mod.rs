use std::collections::HashMap;

use value::StorageValue;

use crate::data::redis_value::list::RedisValueList;
use crate::data::redis_value::set::RedisValueSet;
use crate::data::redis_value::string::RedisValueString;
use crate::data::redis_value::RedisValue;
use crate::data::storage_service::operator_service::storage::expiration_map::ExpirationMap;

pub mod expiration_map;
pub mod value;

#[derive(Default)]
pub struct RedisStorage {
    values: HashMap<String, StorageValue>,
    expirations: ExpirationMap,
}

impl RedisStorage {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, key: String, value: RedisValue) -> Option<RedisValue> {
        let storage_value = StorageValue::new(value);
        let old_value = self.values.insert(key, storage_value);
        old_value.map(|v| v.extract_value())
    }

    pub fn access(&mut self, key: &str) -> Option<&RedisValue> {
        let storage_value = self.values.get_mut(key);
        storage_value.map(|v| v.access())
    }

    pub fn length(&self) -> usize {
        self.values.len()
    }

    pub fn get(&mut self, key: &str) -> Option<&RedisValue> {
        match self.values.get_mut(key) {
            Some(value) => {
                if self.expirations.is_expired(key){
                    None
                }else {
                    Some(value.access())
                }
            }
            None => {
                None
            }
        }
    }

    pub fn mut_get(&mut self, key: &str) -> Option<&mut RedisValue> {
        match self.values.get_mut(key) {
            Some(value) => {
                if self.expirations.is_expired(key){
                    None
                }else {
                    Some(value.access_mut())
                }
            }
            None => {
                None
            }
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        match self.values.get(key) {
            Some(_) => {
                !self.expirations.is_expired(key)
            }
            None => {
                false
            }
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<RedisValue> {
        self.expirations.remove(key);
        self.values.remove(key).map(|mut v|v.access()).cloned()
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
                    storage.insert(parsed_line[0].trim().to_owned(), RedisValue::String(value));
                }
                "list" => {
                    let value = RedisValueList::new(parsed_line[2].trim().to_owned());
                    storage.insert(parsed_line[0].trim().to_owned(), RedisValue::List(value));
                }
                "set" => {
                    let value = RedisValueSet::new(parsed_line[2].trim().to_owned());
                    storage.insert(parsed_line[0].trim().to_owned(), RedisValue::Set(value));
                }
                _ => println!("Data type not supported in deserialization"),
            }
        }
        storage
    }

    /*
    pub fn print(&self) {
        for (key, value) in &self.storage {
            println!("{:?}", key);
            println!("{:?}", value.serialize());
        }
    }
    */
}
