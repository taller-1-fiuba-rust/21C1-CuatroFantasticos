use crate::protocol_serialization::ProtocolSerializer;

pub enum RedisError {
    Nil,
    NilArray,
    None,
    NotAString,
    NotANumber,
    NotAList,
    NotASet,
    NotAListNorSet,
    NonExistent,
    NotASetOfNumbers,
    NotAListOfNumbers,
    Unknown,
    Existent,
    NotVolatil,
}

impl ProtocolSerializer for RedisError {
    fn protocol_serialize_to_simple_string(&self) -> String {
        match self {
            RedisError::Nil => String::from("$-1\r\n"),
            RedisError::NilArray => String::from("*-1\r\n"),
            RedisError::None => String::from("-NONE\r\n"),
            RedisError::NotAString => {
                String::from("-WRONGTYPE The key does not store a String\r\n")
            }
            RedisError::NotANumber => {
                String::from("-ERR value is not an integer or out of range\r\n")
            }
            RedisError::NotAList => String::from("-WRONGTYPE The key does not store a List\r\n"),
            RedisError::NotASet => String::from("-WRONGTYPE The key does not store a Set\r\n"),
            RedisError::NotAListNorSet => {
                String::from("-WRONGTYPE The key does not store a List nor a Set\r\n")
            }
            RedisError::NotASetOfNumbers => {
                String::from("-WRONGTYPE The key does not store a Set of numbers\r\n")
            }
            RedisError::NotAListOfNumbers => {
                String::from("-WRONGTYPE The key does not store a List of numbers\r\n")
            }
            RedisError::Unknown => String::from("-UNKNOWN something went wrong\r\n"),
            RedisError::NonExistent => String::from("-NONEXISTENT The key does not exist\r\n"),
            RedisError::Existent => String::from("-EXISTENT The key already exists\r\n"),
            RedisError::NotVolatil => String::from("-NOTVOLATIL The key is not volatil\r\n"),
        }
    }

    fn protocol_serialize_to_int(&self) -> String {
        self.protocol_serialize_to_simple_string()
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        self.protocol_serialize_to_simple_string()
    }
}
