use crate::configuration::Configuration;
use crate::protocol_serialization::ProtocolSerializer;
use std::fmt::Display;

pub struct ConfResponseMessage {
    value_response: ConfResult,
}

impl ConfResponseMessage {
    pub fn new(value_response: ConfResult) -> ConfResponseMessage {
        ConfResponseMessage { value_response }
    }

    pub fn get_value(&self) -> &ConfResult {
        &self.value_response
    }
}

pub enum ConfResult {
    Ok,
    Vector(Vec<String>),
    OkConf(Configuration),
    Error(ConfError),
}

pub enum ConfError {
    NonExistent,
    Unknown,
}

impl ProtocolSerializer for ConfResult {
    fn protocol_serialize_to_simple_string(&self) -> String {
        match self {
            ConfResult::Ok => {
                format!("+{}\r\n", "OK")
            }
            ConfResult::Vector(vec) => vec.protocol_serialize_to_simple_string(),
            ConfResult::OkConf(_) => "-ERR can not get configuration\r\n".to_string(),
            ConfResult::Error(err) => err.protocol_serialize_to_simple_string(),
        }
    }

    fn protocol_serialize_to_int(&self) -> String {
        match self {
            ConfResult::Ok => {
                format!(":{}\r\n", "OK")
            }
            ConfResult::Vector(vec) => vec.protocol_serialize_to_int(),
            ConfResult::OkConf(_) => "-ERR can not get configuration\r\n".to_string(),
            ConfResult::Error(err) => err.protocol_serialize_to_int(),
        }
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        fn bulk_string_formatter<T: Display>(value: T) -> String {
            let string = format!("{}", value);
            let len = string.len();
            format!("${}\r\n{}\r\n", len, value)
        }
        match self {
            ConfResult::Ok => bulk_string_formatter("OK"),
            ConfResult::Vector(vec) => vec.protocol_serialize_to_bulk_string(),
            ConfResult::OkConf(_) => "-ERR can not get configuration\r\n".to_string(),
            ConfResult::Error(err) => err.protocol_serialize_to_bulk_string(),
        }
    }
}

impl ProtocolSerializer for ConfError {
    fn protocol_serialize_to_simple_string(&self) -> String {
        match self {
            ConfError::NonExistent => {
                "-ERR Could not get the value requested because the key did not exist\r\n"
                    .to_string()
            }
            ConfError::Unknown => "-ERR something unexpected happened\r\n".to_string(),
        }
    }

    fn protocol_serialize_to_int(&self) -> String {
        match self {
            ConfError::NonExistent => self.protocol_serialize_to_simple_string(),
            ConfError::Unknown => self.protocol_serialize_to_simple_string(),
        }
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        match self {
            ConfError::NonExistent => self.protocol_serialize_to_simple_string(),
            ConfError::Unknown => self.protocol_serialize_to_simple_string(),
        }
    }
}
