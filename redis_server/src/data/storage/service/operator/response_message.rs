use crate::data::redis_value::RedisValue;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;
use std::borrow::Borrow;
use std::fmt::Display;

pub struct StorageResponseMessage {
    value_response: StorageResult,
}

impl StorageResponseMessage {
    pub fn new(value_response: StorageResult) -> StorageResponseMessage {
        StorageResponseMessage { value_response }
    }

    pub fn get_value(&self) -> &StorageResult {
        &self.value_response
    }
}

pub enum StorageResult {
    Int(i32),
    String(String),
    RedisValue(RedisValue),
    Bool(bool),
    Vector(Vec<String>),
    Ok,
    Error(RedisError),
}

impl ProtocolSerializer for StorageResult {
    fn protocol_serialize_to_simple_string(&self) -> String {
        match self.borrow() {
            StorageResult::Int(value) => format!("+{}\r\n", value),
            StorageResult::String(value) => format!("+{}\r\n", value),
            StorageResult::RedisValue(value) => value.protocol_serialize_to_simple_string(),
            StorageResult::Bool(value) => {
                format!("+{}\r\n", if *value { "1" } else { "0" })
            }
            StorageResult::Vector(vec) => vec.protocol_serialize_to_simple_string(),
            StorageResult::Ok => format!("+{}\r\n", "OK"),
            StorageResult::Error(value) => value.protocol_serialize_to_simple_string(),
        }
    }

    fn protocol_serialize_to_int(&self) -> String {
        match self.borrow() {
            StorageResult::Int(value) => format!(":{}\r\n", value),
            StorageResult::String(value) => format!(":{}\r\n", value),
            StorageResult::RedisValue(value) => value.protocol_serialize_to_int(),
            StorageResult::Bool(value) => {
                format!(":{}\r\n", if *value { "1" } else { "0" })
            }
            StorageResult::Vector(vec) => vec.protocol_serialize_to_int(),
            StorageResult::Ok => format!(":{}\r\n", "OK"),
            StorageResult::Error(value) => value.protocol_serialize_to_int(),
        }
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        fn bulk_string_formatter<T: Display>(value: T) -> String {
            let string = format!("{}", value);
            let len = string.len();
            format!("${}\r\n{}\r\n", len, value)
        }
        match self.borrow() {
            StorageResult::Int(value) => bulk_string_formatter(value),
            StorageResult::String(value) => bulk_string_formatter(value),
            StorageResult::RedisValue(value) => value.protocol_serialize_to_bulk_string(),
            StorageResult::Bool(value) => bulk_string_formatter(if *value { "1" } else { "0" }),
            StorageResult::Vector(vec) => vec.protocol_serialize_to_bulk_string(),
            StorageResult::Ok => bulk_string_formatter("OK"),
            StorageResult::Error(value) => value.protocol_serialize_to_bulk_string(),
        }
    }
}
