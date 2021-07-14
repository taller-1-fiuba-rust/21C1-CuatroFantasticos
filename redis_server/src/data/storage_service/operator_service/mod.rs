use std::sync::mpsc;

use crate::data::redis_value::string::RedisValueString;
use crate::data::redis_value::RedisValue;
use crate::data::storage_service::operator_service::request_message::{
    StorageRequestMessage, StorageRequestMessageEnum,
};
use crate::data::storage_service::operator_service::response_error_enum::ResponseErrorEnum;
use crate::data::storage_service::operator_service::response_message::StorageResponseMessageEnum;
use crate::data::storage_service::operator_service::storage::RedisStorage;
use std::io::Read;

pub mod accessor;
pub mod request_message;
pub mod response_error_enum;
pub mod response_message;
pub mod storage;

pub struct StorageOperatorService {
    storage: RedisStorage,
    receiver: mpsc::Receiver<StorageRequestMessage>,
}

impl StorageOperatorService {
    pub fn new<T>(
        mut persistence_object: T,
        receiver: mpsc::Receiver<StorageRequestMessage>,
    ) -> StorageOperatorService
    where
        T: Read + Send + 'static,
    {
        let mut contents = String::new();
        let read_result = persistence_object.read_to_string(&mut contents);
        let contents = contents;
        let storage = match read_result {
            Ok(_) => RedisStorage::deserialize(contents),
            Err(_) => RedisStorage::new(),
        };
        StorageOperatorService { storage, receiver }
    }

    pub fn init(mut self) {
        for message in self.receiver {
            match message.get_message() {
                StorageRequestMessageEnum::GetDbsize => {
                    let value = self.storage.length();
                    let response = StorageResponseMessageEnum::Int(value);
                    let _ = message.respond(response);
                }
                StorageRequestMessageEnum::FlushDb => {
                    self.storage.clear();
                    let response = StorageResponseMessageEnum::Ok;
                    let _ = message.respond(response);
                }
                StorageRequestMessageEnum::Rename(key, new_key) => {
                    if let Some(value) = self.storage.remove(&key) {
                        self.storage.insert(new_key, value);
                        let response = StorageResponseMessageEnum::Ok;
                        let _ = message.respond(response);
                    } else {
                        let response =
                            StorageResponseMessageEnum::Error(ResponseErrorEnum::NonExistent);
                        let _ = message.respond(response);
                    }
                }
                StorageRequestMessageEnum::Exists(key) => {
                    let value = self.storage.contains_key(&key);
                    let response = StorageResponseMessageEnum::Bool(value);
                    let _ = message.respond(response);
                }
                StorageRequestMessageEnum::Del(key) => {
                    let result = self.storage.contains_key(&key);
                    let response = StorageResponseMessageEnum::Bool(result);
                    self.storage.remove(&key);
                    let _ = message.respond(response);
                }
                StorageRequestMessageEnum::Type(key) => {
                    let value = self.storage.access(&key).cloned();
                    match value {
                        Some(value) => {
                            let response = StorageResponseMessageEnum::RedisValue(value);
                            let _ = message.respond(response);
                        }
                        None => {
                            let response =
                                StorageResponseMessageEnum::Error(ResponseErrorEnum::None);
                            let _ = message.respond(response);
                        }
                    }
                }
                StorageRequestMessageEnum::Get(key) => {
                    let value = self.storage.access(&key).cloned();
                    match value {
                        Some(value) => {
                            let response = StorageResponseMessageEnum::RedisValue(value);
                            let _ = message.respond(response);
                        }
                        None => {
                            let response =
                                StorageResponseMessageEnum::Error(ResponseErrorEnum::NonExistent);
                            let _ = message.respond(response);
                        }
                    }
                }
                StorageRequestMessageEnum::Copy(source_key, destination_key) => {
                    let destination = self.storage.contains_key(&destination_key);
                    if destination {
                        let response =
                            StorageResponseMessageEnum::Error(ResponseErrorEnum::Existent);
                        let _ = message.respond(response);
                    } else {
                        let value = self.storage.access(&source_key).cloned();
                        match value {
                            Some(value) => {
                                self.storage.insert(destination_key, value);
                                let response = StorageResponseMessageEnum::Bool(true);
                                let _ = message.respond(response);
                            }
                            None => {
                                let response = StorageResponseMessageEnum::Bool(false);
                                let _ = message.respond(response);
                            }
                        }
                    }
                }
                StorageRequestMessageEnum::Lindex(key, index) => match self.storage.get(&key) {
                    Some(RedisValue::List(value)) => {
                        let result = value.get_index(index);
                        match result {
                            Some(value) => {
                                let response = StorageResponseMessageEnum::String(value);
                                let _ = message.respond(response);
                            }
                            None => {
                                let response =
                                    StorageResponseMessageEnum::Error(ResponseErrorEnum::Nil);
                                let _ = message.respond(response);
                            }
                        }
                    }
                    Some(_) => {
                        let response =
                            StorageResponseMessageEnum::Error(ResponseErrorEnum::NotAList);
                        let _ = message.respond(response);
                    }
                    None => {
                        let response = StorageResponseMessageEnum::Error(ResponseErrorEnum::Nil);
                        let _ = message.respond(response);
                    }
                },

                StorageRequestMessageEnum::Append(key, new_value) => {
                    match self.storage.mut_get(&key) {
                        Some(RedisValue::String(value)) => {
                            let result = value.append(&new_value);
                            let response = StorageResponseMessageEnum::Int(result.len());
                            let _ = message.respond(response);
                        }
                        Some(_) => {
                            let response =
                                StorageResponseMessageEnum::Error(ResponseErrorEnum::NotAString);
                            let _ = message.respond(response);
                        }
                        None => {
                            self.storage.insert(
                                key,
                                RedisValue::String(RedisValueString::new(new_value.clone())),
                            );
                            let response = StorageResponseMessageEnum::Int(new_value.len());
                            let _ = message.respond(response);
                        }
                    };
                }

                StorageRequestMessageEnum::GetDel(key) => {
                    match self.storage.get(&key) {
                        Some(RedisValue::String(value)) => {
                            let response = StorageResponseMessageEnum::String(value.get_value());
                            self.storage.remove(&key);
                            let _ = message.respond(response);
                        }
                        Some(_) => {
                            let response =
                                StorageResponseMessageEnum::Error(ResponseErrorEnum::NotAString);
                            let _ = message.respond(response);
                        }
                        None => {
                            let response =
                                StorageResponseMessageEnum::Error(ResponseErrorEnum::Nil);
                            let _ = message.respond(response);
                        }
                    };
                }

                StorageRequestMessageEnum::GetSet(key, new_value) => {
                    match self.storage.get(&key) {
                        Some(RedisValue::String(value)) => {
                            let response = StorageResponseMessageEnum::String(value.get_value());
                            self.storage
                                .insert(key, RedisValue::String(RedisValueString::new(new_value)));
                            let _ = message.respond(response);
                        }
                        Some(_) => {
                            let response =
                                StorageResponseMessageEnum::Error(ResponseErrorEnum::NotAString);
                            let _ = message.respond(response);
                        }
                        None => {
                            self.storage
                                .insert(key, RedisValue::String(RedisValueString::new(new_value)));
                            let response =
                                StorageResponseMessageEnum::Error(ResponseErrorEnum::Nil);
                            let _ = message.respond(response);
                        }
                    };
                }

                StorageRequestMessageEnum::Strlen(key) => match self.storage.get(&key) {
                    Some(RedisValue::String(value)) => {
                        let response = StorageResponseMessageEnum::Int(value.length());
                        let _ = message.respond(response);
                    }
                    Some(_) => {
                        let response =
                            StorageResponseMessageEnum::Error(ResponseErrorEnum::NotAString);
                        let _ = message.respond(response);
                    }
                    None => {
                        let response = StorageResponseMessageEnum::Int(0);
                        let _ = message.respond(response);
                    }
                },

                StorageRequestMessageEnum::Llen(key) => match self.storage.get(&key) {
                    Some(RedisValue::List(value)) => {
                        let response = StorageResponseMessageEnum::Int(value.length());
                        let _ = message.respond(response);
                    }
                    Some(_) => {
                        let response =
                            StorageResponseMessageEnum::Error(ResponseErrorEnum::NotAList);
                        let _ = message.respond(response);
                    }
                    None => {
                        let response = StorageResponseMessageEnum::Int(0);
                        let _ = message.respond(response);
                    }
                },

                StorageRequestMessageEnum::ExpirationRound => {
                    todo!()
                }
                StorageRequestMessageEnum::Persist => {
                    todo!()
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
    #[test]
    fn test_create_empty_storage() {}
}
