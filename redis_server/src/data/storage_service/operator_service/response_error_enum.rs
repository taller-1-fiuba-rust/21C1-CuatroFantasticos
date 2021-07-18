use crate::protocol_serialization::ProtocolSerializer;

pub enum RedisErrorEnum {
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
}

impl ProtocolSerializer for RedisErrorEnum {
    fn protocol_serialize_to_simple_string(&self) -> String {
        match self {
            RedisErrorEnum::Nil => String::from("$-1\r\n"),
            RedisErrorEnum::NilArray => String::from("*-1\r\n"),
            RedisErrorEnum::None => String::from("-NONE\r\n"),
            RedisErrorEnum::NotAString => {
                String::from("-WRONGTYPE The key does not store a String\r\n")
            }
            RedisErrorEnum::NotANumber => {
                String::from("-ERR value is not an integer or out of range\r\n")
            }
            RedisErrorEnum::NotAList => {
                String::from("-WRONGTYPE The key does not store a List\r\n")
            }
            RedisErrorEnum::NotASet => String::from("-WRONGTYPE The key does not store a Set\r\n"),
            RedisErrorEnum::NotAListNorSet => {
                String::from("-WRONGTYPE The key does not store a List nor a Set\r\n")
            }
            RedisErrorEnum::NotASetOfNumbers => {
                String::from("-WRONGTYPE The key does not store a Set of numbers\r\n")
            }
            RedisErrorEnum::NotAListOfNumbers => {
                String::from("-WRONGTYPE The key does not store a List of numbers\r\n")
            }
            RedisErrorEnum::Unknown => String::from("-UNKNOWN something went wrong\r\n"),
            RedisErrorEnum::NonExistent => String::from("-NONEXISTENT The key does not exist\r\n"),
            RedisErrorEnum::Existent => String::from("-EXISTENT The key already exists\r\n"),
        }
    }

    fn protocol_serialize_to_int(&self) -> String {
        self.protocol_serialize_to_simple_string()
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        self.protocol_serialize_to_simple_string()
    }
}
