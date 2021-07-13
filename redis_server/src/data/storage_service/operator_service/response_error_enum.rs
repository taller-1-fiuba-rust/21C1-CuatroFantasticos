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
                String::from("-WRONGTYPE The key does not store a String")
            }
            ResponseErrorEnum::NotAList => String::from("-WRONGTYPE The key does not store a List"),
            ResponseErrorEnum::NotASet => String::from("-WRONGTYPE The key does not store a Set"),
            ResponseErrorEnum::NonExistent => String::from("-NONEXISTENT The key does not exist"),
            ResponseErrorEnum::Existent => String::from("-EXISTENT The key already exists"),
        }
    }

    fn protocol_serialize_to_int(&self) -> String {
        self.protocol_serialize_to_simple_string()
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        self.protocol_serialize_to_simple_string()
    }
}
