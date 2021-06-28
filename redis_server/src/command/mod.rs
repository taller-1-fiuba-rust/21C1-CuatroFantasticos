use std::sync::mpsc;
use crate::data::storage_accessor::StorageAccessor;

pub mod dbsize;

pub trait RedisCommand {
    fn execute(&self, accesor: StorageAccessor) -> Result<String, String>;
}
