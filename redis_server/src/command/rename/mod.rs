use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;
use crate::data::storage::response_message::StorageResponseMessageEnum;

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
        let rename = accessor.access(StorageRequestMessageEnum::Rename(
            self.key.clone(),
            self.new_key.clone(),
        ))?;
        let response = match rename.get_value() {
            StorageResponseMessageEnum::ResponseOk => "+OK\r\n".to_string(),
            StorageResponseMessageEnum::ResponseError(message) => {
                format!("-{}\r\n", message)
            }
            _ => "falle jeje".to_string(),
        };
        Ok(response)
    }
}
