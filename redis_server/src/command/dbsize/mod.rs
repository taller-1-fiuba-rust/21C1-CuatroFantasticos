use crate::command::RedisCommand;
use std::sync::mpsc;
use crate::data::storage::Storage;
use crate::data::storage_accessor::StorageAccessor;
use crate::data::storage_message::StorageMessageEnum;

pub struct RedisCommandDbSize {

}

impl RedisCommandDbSize {
    pub fn new() -> RedisCommandDbSize {
        RedisCommandDbSize{
        }
    }
}

impl RedisCommand for RedisCommandDbSize {
    fn execute(&self, accessor: StorageAccessor) -> Result<String,String> {
        accessor.access(StorageMessageEnum::GetDbsize)
    }
}