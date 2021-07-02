use crate::command::RedisCommand;
use crate::data::storage_accessor::StorageAccessor;
use crate::data::storage_message::StorageMessageEnum;

pub struct RedisCommandRename {
    key: String,
    new_key: String,
}

impl RedisCommandRename {
    pub fn new(key: String, new_key: String) -> RedisCommandRename {
        RedisCommandRename { key, new_key }
    }
}

impl RedisCommand for RedisCommandRename {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let rename = accessor.access(StorageMessageEnum::Rename(
            self.key.clone(),
            self.new_key.clone(),
        ))?;
        let response = format!("+{}\r\n", rename);
        Ok(response)
    }
}
