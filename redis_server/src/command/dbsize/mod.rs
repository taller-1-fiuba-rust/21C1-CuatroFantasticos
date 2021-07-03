use crate::command::RedisCommand;
use crate::data::storage_accessor::StorageAccessor;
use crate::data::storage_message::StorageMessageEnum;
use crate::data::storage_response::StorageResponseEnum;

pub struct RedisCommandDbSize {}

impl RedisCommandDbSize {
    pub fn new() -> RedisCommandDbSize {
        RedisCommandDbSize {}
    }
}

impl RedisCommand for RedisCommandDbSize {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let dbsize = accessor.access(StorageMessageEnum::GetDbsize)?;
        let value = match dbsize.get_value() {
            StorageResponseEnum::ResponseInt(value) => Ok(value),
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
