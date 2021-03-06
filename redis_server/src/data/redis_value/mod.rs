use crate::data::redis_value::list::RedisValueList;
use crate::data::redis_value::set::RedisValueSet;
use crate::data::redis_value::string::RedisValueString;
use crate::protocol_serialization::ProtocolSerializer;

pub mod list;
pub mod set;
pub mod string;

#[derive(Debug, Clone)]
pub enum RedisValue {
    List(RedisValueList),
    Set(RedisValueSet),
    String(RedisValueString),
}

impl RedisValue {
    pub fn serialize(&self) -> String {
        match self {
            RedisValue::List(list) => list.serialize(),
            RedisValue::Set(set) => set.serialize(),
            RedisValue::String(string) => string.serialize(),
        }
    }
    pub fn get_type(&self) -> String {
        match self {
            RedisValue::List(list) => list.get_type(),
            RedisValue::Set(set) => set.get_type(),
            RedisValue::String(string) => string.get_type(),
        }
    }
}

impl ProtocolSerializer for RedisValue {
    fn protocol_serialize_to_simple_string(&self) -> String {
        match self {
            RedisValue::List(list) => list.protocol_serialize_to_simple_string(),
            RedisValue::Set(set) => set.protocol_serialize_to_simple_string(),
            RedisValue::String(string) => string.protocol_serialize_to_simple_string(),
        }
    }

    fn protocol_serialize_to_int(&self) -> String {
        match self {
            RedisValue::List(list) => list.protocol_serialize_to_int(),
            RedisValue::Set(set) => set.protocol_serialize_to_int(),
            RedisValue::String(string) => string.protocol_serialize_to_int(),
        }
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        match self {
            RedisValue::List(list) => list.protocol_serialize_to_bulk_string(),
            RedisValue::Set(set) => set.protocol_serialize_to_bulk_string(),
            RedisValue::String(string) => string.protocol_serialize_to_bulk_string(),
        }
    }
}
