use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;
use crate::data::storage::response_message::StorageResponseMessageEnum;

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
        let exists = accessor.access(StorageRequestMessageEnum::Exists(self.key.clone()))?;
        let value = match exists.get_value() {
            StorageResponseMessageEnum::ResponseBool(value) => Ok(if *value { "1" } else { "0" }),
            _ => Err("falle jeje"),
        };
        let response = format!(":{}\r\n", value.unwrap());
        Ok(response)
    }
}
