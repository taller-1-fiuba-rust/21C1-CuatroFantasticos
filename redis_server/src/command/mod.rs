pub trait RedisCommand {
    fn execute(&self);
}
