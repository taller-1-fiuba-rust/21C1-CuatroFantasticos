use crate::protocol_serialization::ProtocolSerializer;

pub enum ResponseErrorEnum {
    Nil,
    None,
    NotAString,
    NotAList,
    NotASet,
    NonExistent,
    Existent,
}

impl ProtocolSerializer for ResponseErrorEnum {
    fn protocol_serialize_to_simple_string(&self) -> String {
        match self {
            ResponseErrorEnum::Nil => String::from("$-1\r\n"),
            ResponseErrorEnum::None => String::from("-NONE\r\n"),
            ResponseErrorEnum::NotAString => {
                String::from("-WRONGTYPE The key does not store a String\r\n")
            }
            ResponseErrorEnum::NotAList => {
                String::from("-WRONGTYPE The key does not store a List\r\n")
            }
            ResponseErrorEnum::NotASet => {
                String::from("-WRONGTYPE The key does not store a Set\r\n")
            }
            ResponseErrorEnum::NonExistent => {
                String::from("-NONEXISTENT The key does not exist\r\n")
            }
            ResponseErrorEnum::Existent => String::from("-EXISTENT The key already exists\r\n"),
        }
    }

    fn protocol_serialize_to_int(&self) -> String {
        self.protocol_serialize_to_simple_string()
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        self.protocol_serialize_to_simple_string()
    }
}
