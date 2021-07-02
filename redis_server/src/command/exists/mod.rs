use crate::command::RedisCommand;
use crate::data::storage_accessor::StorageAccessor;
use crate::data::storage_message::StorageMessageEnum;

pub struct RedisCommandExists {
    key: String,
}

impl RedisCommandExists {
    pub fn new(key: String) -> RedisCommandExists {
        RedisCommandExists { key }
    }
}

impl RedisCommand for RedisCommandExists {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let exists = accessor.access(StorageMessageEnum::Exists(self.key.clone()))?;
        let response = format!(":{}\r\n", exists);
        Ok(response)
    }
}
