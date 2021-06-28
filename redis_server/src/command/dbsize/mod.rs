use crate::command::RedisCommand;
use crate::data::storage_accessor::StorageAccessor;
use crate::data::storage_message::StorageMessageEnum;

pub struct RedisCommandDbSize {}

impl RedisCommandDbSize {
    pub fn new() -> RedisCommandDbSize {
        RedisCommandDbSize {}
    }
}

impl RedisCommand for RedisCommandDbSize {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let dbsize = accessor.access(StorageMessageEnum::GetDbsize)?;
        let response = format!(":{}\r\n", dbsize);
        Ok(response)
    }
}

impl Default for RedisCommandDbSize {
    fn default() -> Self {
        Self::new()
    }
}
