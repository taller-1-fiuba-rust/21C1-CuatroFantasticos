pub trait RedisValue {
    fn serialize(&self) -> String;
}
