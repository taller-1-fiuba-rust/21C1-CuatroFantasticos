use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;
use crate::data::storage::response_message::StorageResponseMessageEnum;

pub struct RedisCommandDel {
    key: String,
}

impl RedisCommandDel {
    pub fn new(key: String) -> RedisCommandDel {
        RedisCommandDel { key }
    }
}

impl RedisCommand for RedisCommandDel {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let rename = accessor.access(StorageRequestMessageEnum::Del(self.key.clone()))?;
        let value = match rename.get_value() {
            StorageResponseMessageEnum::ResponseBool(value) => Ok(if *value { "1" } else { "0" }),
            _ => Err("falle jeje"),
        };
        let response = format!(":{}\r\n", value.unwrap());
        Ok(response)
    }
}
