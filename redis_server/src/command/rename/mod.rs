use crate::command::RedisCommand;
use crate::data::storage_accessor::StorageAccessor;
use crate::data::storage_message::StorageMessageEnum;
use crate::data::storage_response::StorageResponseEnum;

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
        let response = match rename.get_value() {
            StorageResponseEnum::ResponseOk => "+OK\r\n".to_string(),
            StorageResponseEnum::ResponseError(message) => {
                format!("-{}\r\n", message)
            }
            _ => "falle jeje".to_string(),
        };
        Ok(response)
    }
}
