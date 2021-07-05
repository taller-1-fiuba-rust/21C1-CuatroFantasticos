use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;

pub struct RedisCommandPing {}

impl RedisCommandPing {
    pub fn new() -> RedisCommandPing {
        RedisCommandPing {}
    }
}

impl RedisCommand for RedisCommandPing {
    fn execute(&self, _accessor: StorageAccessor) -> Result<String, String> {
        let response = "+PONG\r\n".to_owned();
        Ok(response)
    }
}

impl Default for RedisCommandPing {
    fn default() -> Self {
        Self::new()
    }
}
