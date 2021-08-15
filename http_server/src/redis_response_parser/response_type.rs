pub enum RedisResponseType {
    BulkString,
    SimpleString(String),
    Error(String),
    Array(usize),
    Integer(i64),
    Nil,
}
