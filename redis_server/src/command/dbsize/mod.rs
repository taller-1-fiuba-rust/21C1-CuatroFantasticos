use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;
use crate::data::storage::response_message::StorageResponseMessageEnum;

pub struct RedisCommandDbSize {}

impl RedisCommandDbSize {
    pub fn new() -> RedisCommandDbSize {
        RedisCommandDbSize {}
    }
}

impl RedisCommand for RedisCommandDbSize {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let dbsize = accessor.access(StorageRequestMessageEnum::GetDbsize)?;
        let value = match dbsize.get_value() {
            StorageResponseMessageEnum::ResponseInt(value) => Ok(value),
            _ => Err("falle jeje"),
        };
        let response = format!(":{}\r\n", value.unwrap());
        Ok(response)
    }
}

impl Default for RedisCommandDbSize {
    fn default() -> Self {
        Self::new()
    }
}
