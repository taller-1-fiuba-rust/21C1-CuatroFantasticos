use std::fs::File;
use std::io::{Read, Write};
use std::sync::mpsc;

use crate::data::redis_value::set::RedisValueSet;
use crate::data::redis_value::string::RedisValueString;
use crate::data::redis_value::RedisValue;
use crate::data::storage::service::operator::request_message::{
    StorageAction, StorageRequestMessage,
};
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::data::storage::RedisStorage;

pub mod accessor;
pub mod accessor_builder;
pub mod request_message;
pub mod response_message;
pub mod result_error;

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
                StorageAction::Dbsize => {
                    let value = self.storage.length();
                    let response = StorageResult::Int(value as i32);
                    let _ = message.respond(response);
                }
                StorageAction::FlushDb => {
                    self.storage.clear();
                    let response = StorageResult::Ok;
                    let _ = message.respond(response);
                }

                StorageAction::Keys(pattern) => {
                    let keys = self.storage.keys_by_pattern(&pattern);
                    let response = StorageResult::Vector(keys);
                    let _ = message.respond(response);
                }

                StorageAction::Rename(key, new_key) => {
                    if let Some(value) = self.storage.remove(&key) {
                        self.storage.insert(&new_key, value);
                        let response = StorageResult::Ok;
                        let _ = message.respond(response);
                    } else {
                        let response = StorageResult::Error(RedisError::NonExistent);
                        let _ = message.respond(response);
                    }
                }
                StorageAction::Exists(key) => {
                    let value = self.storage.contains_key(&key);
                    let response = StorageResult::Bool(value);
                    let _ = message.respond(response);
                }
                StorageAction::Del(key) => {
                    let result = self.storage.contains_key(&key);
                    let response = StorageResult::Bool(result);
                    self.storage.remove(&key);
                    let _ = message.respond(response);
                }
                StorageAction::Type(key) => {
                    let value = self.storage.access(&key).cloned();
                    match value {
                        Some(value) => {
                            let response = StorageResult::RedisValue(value);
                            let _ = message.respond(response);
                        }
                        None => {
                            let response = StorageResult::Error(RedisError::None);
                            let _ = message.respond(response);
                        }
                    }
                }
                StorageAction::Touch(key) => {
                    let value = self.storage.access(&key).is_some();
                    let response = StorageResult::Bool(value);
                    let _ = message.respond(response);
                }
                StorageAction::Persist(key) => {
                    let value = self.storage.persist(&key).is_some();
                    let response = StorageResult::Bool(value);
                    let _ = message.respond(response);
                }
                StorageAction::Get(key) => {
                    let value = self.storage.access(&key).cloned();
                    match value {
                        Some(value) => {
                            let response = StorageResult::RedisValue(value);
                            let _ = message.respond(response);
                        }
                        None => {
                            let response = StorageResult::Error(RedisError::NonExistent);
                            let _ = message.respond(response);
                        }
                    }
                }
                StorageAction::Copy(source_key, destination_key) => {
                    let destination = self.storage.contains_key(&destination_key);
                    if destination {
                        let response = StorageResult::Error(RedisError::Existent);
                        let _ = message.respond(response);
                    } else {
                        let value = self.storage.access(&source_key).cloned();
                        match value {
                            Some(value) => {
                                self.storage.insert(&destination_key, value);
                                let response = StorageResult::Bool(true);
                                let _ = message.respond(response);
                            }
                            None => {
                                let response = StorageResult::Bool(false);
                                let _ = message.respond(response);
                            }
                        }
                    }
                }
                StorageAction::Lindex(key, index) => match self.storage.get(&key) {
                    Some(RedisValue::List(value)) => {
                        let result = value.get_index(index);
                        match result {
                            Some(value) => {
                                let response = StorageResult::String(value);
                                let _ = message.respond(response);
                            }
                            None => {
                                let response = StorageResult::Error(RedisError::Nil);
                                let _ = message.respond(response);
                            }
                        }
                    }
                    Some(_) => {
                        let response = StorageResult::Error(RedisError::NotAList);
                        let _ = message.respond(response);
                    }
                    None => {
                        let response = StorageResult::Error(RedisError::Nil);
                        let _ = message.respond(response);
                    }
                },

                StorageAction::Append(key, new_value) => {
                    match self.storage.mut_get(&key) {
                        Some(RedisValue::String(value)) => {
                            let result = value.append(&new_value);
                            let response = StorageResult::Int(result.len() as i32);
                            let _ = message.respond(response);
                        }
                        Some(_) => {
                            let response = StorageResult::Error(RedisError::NotAString);
                            let _ = message.respond(response);
                        }
                        None => {
                            self.storage.insert(
                                &key,
                                RedisValue::String(RedisValueString::new(new_value.clone())),
                            );
                            let response = StorageResult::Int(new_value.len() as i32);
                            let _ = message.respond(response);
                        }
                    };
                }
                StorageAction::Scard(key) => match self.storage.get(&key) {
                    Some(RedisValue::Set(value)) => {
                        let response = StorageResult::Int(value.length() as i32);
                        let _ = message.respond(response);
                    }
                    Some(_) => {
                        let response = StorageResult::Error(RedisError::NotASet);
                        let _ = message.respond(response);
                    }
                    None => {
                        let response = StorageResult::Int(0);
                        let _ = message.respond(response);
                    }
                },

                StorageAction::GetDel(key) => {
                    match self.storage.get(&key) {
                        Some(RedisValue::String(value)) => {
                            let response = StorageResult::String(value.get_value());
                            self.storage.remove(&key);
                            let _ = message.respond(response);
                        }
                        Some(_) => {
                            let response = StorageResult::Error(RedisError::NotAString);
                            let _ = message.respond(response);
                        }
                        None => {
                            let response = StorageResult::Error(RedisError::Nil);
                            let _ = message.respond(response);
                        }
                    };
                }

                StorageAction::GetSet(key, new_value) => {
                    match self.storage.get(&key) {
                        Some(RedisValue::String(value)) => {
                            let response = StorageResult::String(value.get_value());
                            self.storage
                                .insert(&key, RedisValue::String(RedisValueString::new(new_value)));
                            let _ = message.respond(response);
                        }
                        Some(_) => {
                            let response = StorageResult::Error(RedisError::NotAString);
                            let _ = message.respond(response);
                        }
                        None => {
                            self.storage
                                .insert(&key, RedisValue::String(RedisValueString::new(new_value)));
                            let response = StorageResult::Error(RedisError::Nil);
                            let _ = message.respond(response);
                        }
                    };
                }

                StorageAction::DecrBy(key, decr_value) => {
                    match self.storage.mut_get(&key) {
                        Some(RedisValue::String(old_value)) => {
                            match old_value.get_value().parse::<i32>() {
                                Ok(value) => {
                                    let new_value = value - decr_value;
                                    old_value.set_value(new_value.to_string());
                                    let reponse = StorageResult::Int(new_value);
                                    let _ = message.respond(reponse);
                                }
                                Err(_) => {
                                    let response = StorageResult::Error(RedisError::NotANumber);
                                    let _ = message.respond(response);
                                }
                            }
                        }
                        Some(_) => {
                            let response = StorageResult::Error(RedisError::NotAString);
                            let _ = message.respond(response);
                        }
                        None => {
                            let decr_value = -decr_value;
                            self.storage.insert(
                                &key,
                                RedisValue::String(RedisValueString::new(decr_value.to_string())),
                            );
                            let reponse = StorageResult::Int(decr_value);
                            let _ = message.respond(reponse);
                        }
                    };
                }

                StorageAction::Expire(key, expiration) => {
                    if self.storage.contains_key(&key) {
                        self.storage.expire(&key, expiration * 1000);
                        let response = StorageResult::Bool(true);
                        let _ = message.respond(response);
                    } else {
                        let response = StorageResult::Bool(false);
                        let _ = message.respond(response);
                    }
                }

                StorageAction::Sismember(key, member) => match self.storage.get(&key) {
                    Some(RedisValue::Set(value)) => {
                        let response = StorageResult::Bool(value.contains(member));
                        let _ = message.respond(response);
                    }
                    Some(_) => {
                        let response = StorageResult::Error(RedisError::NotASet);
                        let _ = message.respond(response);
                    }
                    None => {
                        let response = StorageResult::Bool(false);
                        let _ = message.respond(response);
                    }
                },

                StorageAction::IncrBy(key, incr_value) => {
                    match self.storage.mut_get(&key) {
                        Some(RedisValue::String(old_value)) => {
                            match old_value.get_value().parse::<i32>() {
                                Ok(value) => {
                                    let new_value = value + incr_value;
                                    old_value.set_value(new_value.to_string());
                                    let reponse = StorageResult::Int(new_value);
                                    let _ = message.respond(reponse);
                                }
                                Err(_) => {
                                    let response = StorageResult::Error(RedisError::NotANumber);
                                    let _ = message.respond(response);
                                }
                            }
                        }
                        Some(_) => {
                            let response = StorageResult::Error(RedisError::NotAString);
                            let _ = message.respond(response);
                        }
                        None => {
                            let incr_value = incr_value;
                            self.storage.insert(
                                &key,
                                RedisValue::String(RedisValueString::new(incr_value.to_string())),
                            );
                            let reponse = StorageResult::Int(incr_value);
                            let _ = message.respond(reponse);
                        }
                    };
                }

                StorageAction::Strlen(key) => match self.storage.get(&key) {
                    Some(RedisValue::String(value)) => {
                        let response = StorageResult::Int(value.length() as i32);
                        let _ = message.respond(response);
                    }
                    Some(_) => {
                        let response = StorageResult::Error(RedisError::NotAString);
                        let _ = message.respond(response);
                    }
                    None => {
                        let response = StorageResult::Int(0);
                        let _ = message.respond(response);
                    }
                },

                StorageAction::Llen(key) => match self.storage.access(&key) {
                    Some(RedisValue::List(value)) => {
                        let response = StorageResult::Int(value.length() as i32);
                        let _ = message.respond(response);
                    }
                    Some(_) => {
                        let response = StorageResult::Error(RedisError::NotAList);
                        let _ = message.respond(response);
                    }
                    None => {
                        let response = StorageResult::Int(0);
                        let _ = message.respond(response);
                    }
                },

                StorageAction::SAdd(key, members) => match self.storage.mut_get(&key) {
                    Some(RedisValue::Set(value)) => {
                        let mut members_added = 0;
                        for member in members {
                            members_added += value.add(member);
                        }
                        let response = StorageResult::Int(members_added);
                        let _ = message.respond(response);
                    }
                    Some(_) => {
                        let response = StorageResult::Error(RedisError::NotASet);
                        let _ = message.respond(response);
                    }
                    None => {
                        let mut new_set = RedisValueSet::new();
                        let mut members_added = 0;
                        for member in members {
                            members_added += new_set.add(member);
                        }
                        self.storage.insert(&key, RedisValue::Set(new_set));
                        let response = StorageResult::Int(members_added);
                        let _ = message.respond(response);
                    }
                },

                StorageAction::ExpireAt(key, expiration) => {
                    if self.storage.contains_key(&key) {
                        self.storage.expire_at(&key, expiration);
                        let response = StorageResult::Bool(true);
                        let _ = message.respond(response);
                    } else {
                        let response = StorageResult::Bool(false);
                        let _ = message.respond(response);
                    }
                }

                StorageAction::Ttl(key) => {
                    if self.storage.contains_key(&key) {
                        let response = match self.storage.ttl(&key) {
                            Some(value) => StorageResult::Int((value / 1000) as i32),
                            None => StorageResult::Error(RedisError::NotVolatile),
                        };
                        let _ = message.respond(response);
                    } else {
                        let response = StorageResult::Error(RedisError::NonExistent);
                        let _ = message.respond(response);
                    }
                }

                StorageAction::Set(key, value) => {
                    self.storage.insert(
                        &key,
                        RedisValue::String(RedisValueString::new(value.clone())),
                    );
                    let response = StorageResult::Ok;
                    let _ = message.respond(response);
                }

                StorageAction::MSet(keys, values) => {
                    for (key, value) in keys.iter().zip(values.iter()) {
                        self.storage.insert(
                            key,
                            RedisValue::String(RedisValueString::new(value.clone())),
                        );
                    }
                    let response = StorageResult::Ok;
                    let _ = message.respond(response);
                }

                StorageAction::MGet(keys) => {
                    let mut values = Vec::new();
                    for key in keys {
                        match self.storage.access(&key) {
                            Some(RedisValue::String(value)) => values.push(Some(value.serialize())),
                            _ => values.push(None),
                        };
                    }
                    let response = StorageResult::OptionVector(values);
                    let _ = message.respond(response);
                }

                StorageAction::ExpirationRound => {
                    self.storage.clean_partial_expiration();
                    let _ = message.respond(StorageResult::Ok);
                }

                StorageAction::Save => {
                    let mut file = File::create("./dump.rdb").expect("could not create file");
                    for line in self.storage.serialize() {
                        let _ = file.write(&line.as_bytes());
                    }
                    let _ = message.respond(StorageResult::Ok);
                }
                StorageAction::Terminate => {
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
