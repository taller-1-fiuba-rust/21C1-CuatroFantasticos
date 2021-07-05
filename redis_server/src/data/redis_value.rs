pub trait RedisValue: Send {
    fn serialize(&self) -> String;
    fn get_type(&self) -> String;
}
