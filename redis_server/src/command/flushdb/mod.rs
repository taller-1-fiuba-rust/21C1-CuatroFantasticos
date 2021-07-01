use crate::command::RedisCommand;
use crate::data::storage_accessor::StorageAccessor;
use crate::data::storage_message::StorageMessageEnum;

pub struct RedisCommandFlushDb {}

impl RedisCommandFlushDb {
    pub fn new() -> RedisCommandFlushDb {
        RedisCommandFlushDb {}
    }
}

impl RedisCommand for RedisCommandFlushDb {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let flush_db = accessor.access(StorageMessageEnum::FlushDb)?;
        let response = format!("+{}\r\n", flush_db);
        Ok(response)
    }
}

impl Default for RedisCommandFlushDb {
    fn default() -> Self {
        Self::new()
    }
}
