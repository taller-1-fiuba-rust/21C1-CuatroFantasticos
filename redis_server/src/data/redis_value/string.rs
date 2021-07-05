use crate::data::redis_value::RedisValue;
use crate::protocol_serialization::ProtocolSerializer;

#[derive(Clone)]
pub struct RedisValueString {
    contents: String,
}

impl RedisValue for RedisValueString {
    fn serialize(&self) -> String {
        self.contents.clone()
    }

    fn get_type(&self) -> String {
        String::from("String")
    }
}

impl ProtocolSerializer for RedisValueString {
    fn protocol_serialize_to_simple_string(&self) -> String {
        todo!()
    }

    fn protocol_serialize_to_int(&self) -> String {
        todo!()
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        todo!()
    }
}

impl RedisValueString {
    pub fn new(contents: String) -> RedisValueString {
        RedisValueString { contents }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::redis_value::string::RedisValueString;
    use crate::data::redis_value::RedisValue;

    #[test]
    fn test_create_empty_redis_value() {
        let string = String::from("");
        let redis_value_string = RedisValueString::new(string.clone());
        assert_eq!(redis_value_string.serialize(), string);
    }

    #[test]
    fn test_create_redis_value() {
        let string = String::from("hola");
        let redis_value_string = RedisValueString::new(string.clone());
        assert_eq!(redis_value_string.serialize(), string);
    }
}
