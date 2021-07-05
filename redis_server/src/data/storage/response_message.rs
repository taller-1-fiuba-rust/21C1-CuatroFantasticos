use crate::data::redis_value::RedisValue;
use crate::protocol_serialization::ProtocolSerializer;
use std::borrow::Borrow;
use std::fmt::Display;

pub struct StorageResponseMessage {
    value_response: StorageResponseMessageEnum,
}

impl StorageResponseMessage {
    pub fn new(value_response: StorageResponseMessageEnum) -> StorageResponseMessage {
        StorageResponseMessage { value_response }
    }

    pub fn get_value(&self) -> &StorageResponseMessageEnum {
        &self.value_response
    }
}

pub enum StorageResponseMessageEnum {
    Int(usize),
    String(String),
    RedisValue(Box<dyn RedisValue>),
    Bool(bool),
    Ok,
    Error(String),
}

impl ProtocolSerializer for StorageResponseMessageEnum {
    fn protocol_serialize_to_simple_string(&self) -> String {
        match self.borrow() {
            StorageResponseMessageEnum::Int(value) => format!("+{}\r\n", value),
            StorageResponseMessageEnum::String(value) => format!("+{}\r\n", value),
            StorageResponseMessageEnum::RedisValue(value) => {
                value.protocol_serialize_to_simple_string()
            }
            StorageResponseMessageEnum::Bool(value) => {
                format!("+{}\r\n", if *value { "1" } else { "0" })
            }
            StorageResponseMessageEnum::Ok => format!("+{}\r\n", "OK"),
            StorageResponseMessageEnum::Error(value) => format!("+{}\r\n", value),
        }
    }

    fn protocol_serialize_to_int(&self) -> String {
        match self.borrow() {
            StorageResponseMessageEnum::Int(value) => format!(":{}\r\n", value),
            StorageResponseMessageEnum::String(value) => format!(":{}\r\n", value),
            StorageResponseMessageEnum::RedisValue(value) => value.protocol_serialize_to_int(),
            StorageResponseMessageEnum::Bool(value) => {
                format!(":{}\r\n", if *value { "1" } else { "0" })
            }
            StorageResponseMessageEnum::Ok => format!(":{}\r\n", "OK"),
            StorageResponseMessageEnum::Error(value) => format!(":{}\r\n", value),
        }
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        fn bulk_string_formatter<T: Display>(value: T) -> String {
            let string = format!("{}", value);
            let len = string.len();
            format!("${}\r\n{}\r\n", len, value)
        }
        match self.borrow() {
            StorageResponseMessageEnum::Int(value) => bulk_string_formatter(value),
            StorageResponseMessageEnum::String(value) => bulk_string_formatter(value),
            StorageResponseMessageEnum::RedisValue(value) => {
                value.protocol_serialize_to_bulk_string()
            }
            StorageResponseMessageEnum::Bool(value) => {
                bulk_string_formatter(if *value { "1" } else { "0" })
            }
            StorageResponseMessageEnum::Ok => bulk_string_formatter("OK"),
            StorageResponseMessageEnum::Error(value) => bulk_string_formatter(value),
        }
    }
}
