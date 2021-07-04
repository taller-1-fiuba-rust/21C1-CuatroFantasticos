use crate::command::RedisCommand;
use crate::data::storage_accessor::StorageAccessor;
use crate::data::storage_message::StorageMessageEnum;
use crate::data::storage_response::StorageResponseEnum;

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
        let rename = accessor.access(StorageMessageEnum::Del(self.key.clone()))?;
        let value = match rename.get_value() {
            StorageResponseEnum::ResponseBool(value) => Ok(if *value { "1" } else { "0" }),
            _ => Err("falle jeje"),
        };
        let response = format!(":{}\r\n", value.unwrap());
        Ok(response)
    }
}
