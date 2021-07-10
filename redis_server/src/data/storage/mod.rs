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
    storage: HashMap<String, RedisValue>,
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

    pub fn deserialize(contents: String) -> HashMap<String, RedisValue> {
        let mut storage: HashMap<String, RedisValue> = HashMap::new();
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

    pub fn deserialize_empty() -> HashMap<String, RedisValue> {
        let storage: HashMap<String, RedisValue> = HashMap::new();
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
                        StorageResponseMessage::new(StorageResponseMessageEnum::Int(value));
                    let _ = message.respond(response);
                }
                StorageRequestMessageEnum::FlushDb => {
                    self.storage.clear();
                    let response = StorageResponseMessage::new(StorageResponseMessageEnum::Ok);
                    let _ = message.respond(response);
                }
                StorageRequestMessageEnum::Rename(key, new_key) => {
                    if let Some(value) = self.storage.remove(&key) {
                        self.storage.insert(new_key, value);
                        let response = StorageResponseMessage::new(StorageResponseMessageEnum::Ok);
                        let _ = message.respond(response);
                    } else {
                        let response = StorageResponseMessage::new(
                            StorageResponseMessageEnum::Error("The key doesnt exist".to_string()),
                        );
                        let _ = message.respond(response);
                    }
                }
                StorageRequestMessageEnum::Append(key, new_value) => {
                    let result = match self.storage.get_mut(&key) {
                        Some(RedisValue::String(value)) => value.append(&new_value),
                        _ => {
                            self.storage.insert(
                                key,
                                RedisValue::String(RedisValueString::new(new_value.clone())),
                            );
                            new_value
                        }
                    };
                    let response =
                        StorageResponseMessage::new(StorageResponseMessageEnum::Int(result.len()));
                    let _ = message.respond(response);
                }

                StorageRequestMessageEnum::Strlen(key) => {
                    if let Some(value) = self.storage.get(&key).cloned() {
                        let response = StorageResponseMessage::new(
                            StorageResponseMessageEnum::Int(value.serialize().len()),
                        );
                        let _ = message.respond(response);
                    } else {
                        let response =
                            StorageResponseMessage::new(StorageResponseMessageEnum::Int(0));
                        let _ = message.respond(response);
                    }
                }

                StorageRequestMessageEnum::Exists(key) => {
                    let value = self.storage.contains_key(&key);
                    let response =
                        StorageResponseMessage::new(StorageResponseMessageEnum::Bool(value));
                    let _ = message.respond(response);
                }

                StorageRequestMessageEnum::Del(key) => {
                    let result = self.storage.contains_key(&key);
                    let response =
                        StorageResponseMessage::new(StorageResponseMessageEnum::Bool(result));
                    self.storage.remove(&key);
                    let _ = message.respond(response);
                }

                StorageRequestMessageEnum::Type(key) => {
                    let value = self.storage.get(&key).cloned();
                    match value {
                        Some(value) => {
                            let response = StorageResponseMessage::new(
                                StorageResponseMessageEnum::RedisValue(value),
                            );
                            let _ = message.respond(response);
                        }
                        None => {
                            let response = StorageResponseMessage::new(
                                StorageResponseMessageEnum::String(String::from("none")),
                            );
                            let _ = message.respond(response);
                        }
                    }
                }

                StorageRequestMessageEnum::Get(key) => {
                    let value = self.storage.get(&key).cloned();
                    match value {
                        Some(value) => {
                            let response = StorageResponseMessage::new(
                                StorageResponseMessageEnum::RedisValue(value),
                            );
                            let _ = message.respond(response);
                        }
                        None => {
                            let response =
                                StorageResponseMessage::new(StorageResponseMessageEnum::Error(
                                    String::from("Key does not exist"),
                                ));
                            let _ = message.respond(response);
                        }
                    }
                }
                StorageRequestMessageEnum::GetSet(key, new_value) => {
                    match self.storage.get(&key) {
                        Some(RedisValue::String(value)) => {
                            let response = StorageResponseMessage::new(
                                StorageResponseMessageEnum::String(value.get_value()),
                            );
                            self.storage
                                .insert(key, RedisValue::String(RedisValueString::new(new_value)));
                            let _ = message.respond(response);
                        }
                        Some(_) => {
                            let response =
                                StorageResponseMessage::new(StorageResponseMessageEnum::Error(
                                    String::from("Value is not of type String"),
                                ));
                            let _ = message.respond(response);
                        }
                        None => {
                            self.storage
                                .insert(key, RedisValue::String(RedisValueString::new(new_value)));
                            let response = StorageResponseMessage::new(
                                StorageResponseMessageEnum::String(String::from("nil")),
                            );
                            let _ = message.respond(response);
                        }
                    };
                }

                StorageRequestMessageEnum::GetDel(key) => {
                    match self.storage.get(&key) {
                        Some(RedisValue::String(value)) => {
                            let response = StorageResponseMessage::new(
                                StorageResponseMessageEnum::String(value.get_value()),
                            );
                            self.storage.remove(&key);
                            let _ = message.respond(response);
                        }
                        Some(_) => {
                            let response =
                                StorageResponseMessage::new(StorageResponseMessageEnum::Error(
                                    String::from("Value is not of type String"),
                                ));
                            let _ = message.respond(response);
                        }
                        None => {
                            let response = StorageResponseMessage::new(
                                StorageResponseMessageEnum::String(String::from("nil")),
                            );
                            let _ = message.respond(response);
                        }
                    };
                }

                StorageRequestMessageEnum::Copy(source_key, destination_key) => {
                    let destination = self.storage.contains_key(&destination_key);
                    if destination {
                        let response =
                            StorageResponseMessage::new(StorageResponseMessageEnum::Error(
                                String::from("The destination key already exists"),
                            ));
                        let _ = message.respond(response);
                    } else {
                        let value = self.storage.get(&source_key).cloned();
                        match value {
                            Some(value) => {
                                self.storage.insert(destination_key, value);
                                let response = StorageResponseMessage::new(
                                    StorageResponseMessageEnum::Bool(true),
                                );
                                let _ = message.respond(response);
                            }
                            None => {
                                let response = StorageResponseMessage::new(
                                    StorageResponseMessageEnum::Bool(false),
                                );
                                let _ = message.respond(response);
                            }
                        }
                    }
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
