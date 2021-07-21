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

const KEY: usize = 0;
const EXPIRATION: usize = 1;
const LAST_ACCESS_TIME: usize = 2;
const TYPE: usize = 3;
const VALUE: usize = 4;

#[derive(Debug, Default)]
pub struct RedisStorage {
    values: HashMap<String, StorageValue>,
    expirations: ExpirationMap,
}

impl RedisStorage {
    pub fn new() -> Self {
        Default::default()
    }

    /// this function is in charge of removing a key if ir expires
    fn clean_if_expirated(&mut self, key: &str) {
        if self.values.contains_key(key) && self.expirations.is_expired(key) {
            let _ = self.values.remove(key);
            let _ = self.expirations.remove(key);
        }
    }
    /// this function is in charge of inserting a new value to an existing key, if the key does not exists it creates it
    pub fn insert(&mut self, key: &str, value: RedisValue) -> Option<RedisValue> {
        self.clean_if_expirated(key);
        let storage_value = StorageValue::new(value);
        let old_value = self.values.insert(key.to_string(), storage_value);
        let _ = self.expirations.remove(key);
        old_value.map(|v| v.extract_value())
    }
    /// this function is in charge of inserting a value with last acces time
    pub fn insert_with_last_access_time(
        &mut self,
        key: &str,
        value: RedisValue,
        last_access_time: u128,
    ) -> Option<RedisValue> {
        self.clean_if_expirated(key);
        let mut storage_value = StorageValue::new(value);
        storage_value.set_last_access_time(last_access_time);
        let old_value = self.values.insert(key.to_string(), storage_value);
        let _ = self.expirations.remove(key);
        old_value.map(|v| v.extract_value())
    }
    /// this function is in charge of updating the value of a key
    pub fn update(&mut self, key: &str, value: RedisValue) -> Option<RedisValue> {
        self.clean_if_expirated(key);
        if !self.values.contains_key(key) {
            return None;
        }
        let storage_value = StorageValue::new(value);
        let old_value = self.values.insert(key.to_string(), storage_value);
        old_value.map(|v| v.extract_value())
    }
    /// this function is in charge of accessing the storage
    pub fn access(&mut self, key: &str) -> Option<&RedisValue> {
        self.clean_if_expirated(key);
        let storage_value = self.values.get_mut(key);
        storage_value.map(|v| v.access())
    }
    /// this function is in charge of returning the total lenght of the storage
    pub fn length(&self) -> usize {
        self.values.len()
    }

    /// this function is in charge of geting the key's value
    pub fn get(&mut self, key: &str) -> Option<&RedisValue> {
        self.clean_if_expirated(key);
        self.values.get_mut(key).map(|value| value.access())
    }

    pub fn mut_get(&mut self, key: &str) -> Option<&mut RedisValue> {
        self.clean_if_expirated(key);
        self.values.get_mut(key).map(|value| value.access_mut())
    }
    /// this function returns if the storage contains the key or not
    pub fn contains_key(&mut self, key: &str) -> bool {
        self.clean_if_expirated(key);
        self.values.contains_key(key)
    }
    /// this function is in charge of removing the key from storage
    pub fn remove(&mut self, key: &str) -> Option<RedisValue> {
        self.clean_if_expirated(key);
        self.values.remove(key).map(|v| v.extract_value())
    }
    /// this function is in charge of persisting the value of a key
    pub fn persist(&mut self, key: &str) -> Option<u128> {
        self.clean_if_expirated(key);
        self.expirations.remove(key)
    }
    /// this function is in charge of clearing the experation of the values
    pub fn clear(&mut self) {
        self.values.clear();
        self.expirations.clear();
    }
    /// this function is in charge of setting the expire time of a key

    pub fn expire(&mut self, key: &str, ms: u128) {
        self.expirations.expire_in(key.to_string(), ms);
    }
    /// this function is in charge of setting the expire time of a key
    pub fn expire_at(&mut self, key: &str, ms: u128) {
        self.clean_if_expirated(key);
        self.expirations.expire_at(key.to_string(), ms);
    }
    /// this function is in charge of serializing values
    pub fn serialize(&self) -> Vec<String> {
        let mut contents = Vec::new();
        for (key, value) in &self.values {
            let actual_value = value.peek();
            let expiration = self.expirations.get(key).unwrap_or(0);
            let line = format!(
                "{}: {}: {}: {}: {}\n",
                key,
                expiration,
                value.last_access_time(),
                actual_value.get_type(),
                actual_value.serialize()
            );
            contents.push(line);
        }
        contents
    }
    /// this function is in charge of deserializing values
    pub fn deserialize(contents: String) -> RedisStorage {
        let mut storage = RedisStorage::new();
        for line in contents.lines() {
            let split = line.split(':');
            let parsed_line: Vec<&str> = split.collect();
            let key = parsed_line[KEY].trim();
            let last_access_time = parsed_line[LAST_ACCESS_TIME]
                .trim()
                .parse::<u128>()
                .unwrap();
            match parsed_line[TYPE].trim() {
                "String" => {
                    let value = RedisValueString::new(parsed_line[VALUE].trim().to_owned());
                    storage.insert_with_last_access_time(
                        key,
                        RedisValue::String(value),
                        last_access_time,
                    );
                }
                "List" => {
                    let value = RedisValueList::new(parsed_line[VALUE].trim().to_owned());
                    storage.insert_with_last_access_time(
                        key,
                        RedisValue::List(value),
                        last_access_time,
                    );
                }
                "Set" => {
                    let value =
                        RedisValueSet::new_with_contents(parsed_line[VALUE].trim().to_owned());
                    storage.insert_with_last_access_time(
                        key.trim(),
                        RedisValue::Set(value),
                        last_access_time,
                    );
                }
                _ => println!("Data type not supported in deserialization"),
            }
            let expiration = parsed_line[EXPIRATION].trim().parse::<u128>().unwrap();
            if expiration > 0 {
                let _ = storage.expirations.expire_at(key.to_owned(), expiration);
            }
        }
        storage
    }
}
