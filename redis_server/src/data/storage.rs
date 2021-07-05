use crate::data::redis_value::RedisValue;
use crate::data::redis_value_list::RedisValueList;
use crate::data::redis_value_set::RedisValueSet;
use crate::data::redis_value_string::RedisValueString;
use crate::data::storage_message::{StorageMessage, StorageMessageEnum};
use crate::data::storage_response::{StorageResponse, StorageResponseEnum};
use std::collections::HashMap;
use std::fs;
use std::sync::mpsc;
use std::ops::Deref;

pub struct Storage {
    storage: HashMap<String, Box<dyn RedisValue>>,
    receiver: mpsc::Receiver<StorageMessage>,
}

impl Storage {
    pub fn new(filename: &str, receiver: mpsc::Receiver<StorageMessage>) -> Storage {
        let contents = fs::read_to_string(filename);
        let storage = match contents {
            Ok(contents) => Storage::deserialize(contents),
            Err(_) => Storage::deserialize_empty(),
        };
        Storage { storage, receiver }
    }

    pub fn deserialize(contents: String) -> HashMap<String, Box<dyn RedisValue>> {
        let mut storage: HashMap<String, Box<dyn RedisValue>> = HashMap::new();
        for line in contents.lines() {
            let split = line.split(':');
            let parsed_line: Vec<&str> = split.collect();
            match parsed_line[1].trim() {
                "string" => {
                    let value = RedisValueString::new(parsed_line[2].trim().to_owned());
                    storage.insert(parsed_line[0].trim().to_owned(), Box::new(value));
                }
                "list" => {
                    let value = RedisValueList::new(parsed_line[2].trim().to_owned());
                    storage.insert(parsed_line[0].trim().to_owned(), Box::new(value));
                }
                "set" => {
                    let value = RedisValueSet::new(parsed_line[2].trim().to_owned());
                    storage.insert(parsed_line[0].trim().to_owned(), Box::new(value));
                }
                _ => println!("Data type not supported in deserialization"),
            }
        }
        storage
    }

    pub fn deserialize_empty() -> HashMap<String, Box<dyn RedisValue>> {
        let storage: HashMap<String, Box<dyn RedisValue>> = HashMap::new();
        storage
    }

    pub fn serialize(&self) -> Vec<String> {
        let mut contents = Vec::new();
        for (key, value) in &self.storage {
            let line = format!("{}: {}", key, value.serialize());
            contents.push(line);
        }
        contents
    }

    pub fn print(&self) {
        for (key, value) in &self.storage {
            println!("{:?}", key);
            println!("{:?}", value.serialize());
        }
    }

    pub fn get_dbsize(&self) -> usize {
        self.storage.len()
    }

    pub fn init(mut self) {
        for message in self.receiver {
            match message.get_message() {
                StorageMessageEnum::GetDbsize => {
                    let value = self.storage.len();
                    let response = StorageResponse::new(StorageResponseEnum::ResponseInt(value));
                    message
                        .get_sender()
                        .send(response)
                        .expect("Client thread is not listening to storage response");
                }
                StorageMessageEnum::FlushDb => {
                    self.storage.clear();
                    let response = StorageResponse::new(StorageResponseEnum::ResponseBool(true));
                    message
                        .get_sender()
                        .send(response)
                        .expect("Client thread is not listening to storage response");
                }
                StorageMessageEnum::Exists(key) => {
                    let value = self.storage.contains_key(&key);
                    let response = StorageResponse::new(StorageResponseEnum::ResponseBool(value));
                    message
                        .get_sender()
                        .send(response)
                        .expect("Client thread is not listening to storage response")
                }
                StorageMessageEnum::Type(key) => {
                    let value = self.storage.get(&key);
                    match value {
                        Some(value) => {
                            message
                                .get_sender()
                                .send(StorageResponse::new(StorageResponseEnum::ResponseRedisValue(value)))
                                .expect("Client thread is not listening to storage response");
                        }
                        None => {
                            message
                                .get_sender()
                                .send(StorageResponse::new(StorageResponseEnum::ResponseString(String::from("none"))))
                                .expect("Client thread is not listening to storage response");
                        }
                    }
                }
                StorageMessageEnum::Terminate => {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::storage::Storage;

    #[test]
    fn test_create_empty_storage() {
        let storage = Storage::deserialize_empty();
        assert!(storage.is_empty());
    }
}
