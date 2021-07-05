use std::collections::HashMap;
use std::fs;
use std::sync::mpsc;

use crate::data::redis_value::list::RedisValueList;
use crate::data::redis_value::set::RedisValueSet;
use crate::data::redis_value::string::RedisValueString;
use crate::data::redis_value::RedisValue;
use crate::data::storage::request_message::{StorageRequestMessage, StorageRequestMessageEnum};
use crate::data::storage::response_message::{StorageResponseMessage, StorageResponseMessageEnum};

pub mod accessor;
pub mod request_message;
pub mod response_message;

pub struct Storage {
    storage: HashMap<String, Box<dyn RedisValue>>,
    receiver: mpsc::Receiver<StorageRequestMessage>,
}

impl Storage {
    pub fn new(filename: &str, receiver: mpsc::Receiver<StorageRequestMessage>) -> Storage {
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
                StorageRequestMessageEnum::GetDbsize => {
                    let value = self.storage.len();
                    let response =
                        StorageResponseMessage::new(StorageResponseMessageEnum::ResponseInt(value));
                    message
                        .get_sender()
                        .send(response)
                        .expect("Client thread is not listening to storage response");
                }
                StorageRequestMessageEnum::FlushDb => {
                    self.storage.clear();
                    let response =
                        StorageResponseMessage::new(StorageResponseMessageEnum::ResponseOk);
                    message
                        .get_sender()
                        .send(response)
                        .expect("Client thread is not listening to storage response");
                }
                StorageRequestMessageEnum::Rename(key, newkey) => {
                    if let Some(value) = self.storage.remove(&key) {
                        self.storage.insert(newkey, value);
                        let response =
                            StorageResponseMessage::new(StorageResponseMessageEnum::ResponseOk);
                        message
                            .get_sender()
                            .send(response)
                            .expect("Client thread is not listening to storage response")
                    }
                    let response =
                        StorageResponseMessage::new(StorageResponseMessageEnum::ResponseError(
                            "The key doesnt exist".to_string(),
                        ));
                    message
                        .get_sender()
                        .send(response)
                        .expect("Client thread is not listening to storage response")
                }
                StorageRequestMessageEnum::Exists(key) => {
                    let value = self.storage.contains_key(&key);
                    let response = StorageResponseMessage::new(
                        StorageResponseMessageEnum::ResponseBool(value),
                    );
                    message
                        .get_sender()
                        .send(response)
                        .expect("Client thread is not listening to storage response")
                }
                StorageRequestMessageEnum::Terminate => {
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
