pub trait RedisValue {
    fn serialize(&self) -> String;
    fn get_type(&self) -> String;
}
