use crate::command::RedisCommand;
use crate::data::storage_accessor::StorageAccessor;
use crate::data::storage_message::StorageMessageEnum;

pub struct RedisCommandType {
    key: String,
}

impl RedisCommandType {
    pub fn new(key: String) -> RedisCommandType {
        RedisCommandType { key }
    }
}

impl RedisCommand for RedisCommandType {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let flush_db = accessor.access(StorageMessageEnum::Type(self.key.clone()))?;
        let response = format!("+{}\r\n", flush_db);
        Ok(response)
    }
}
