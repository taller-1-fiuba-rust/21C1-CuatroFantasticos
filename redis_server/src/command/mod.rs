use crate::data::storage_accessor::StorageAccessor;

pub mod dbsize;
pub mod exists;
pub mod flushdb;
pub mod ping;
pub mod rename;

pub trait RedisCommand {
    fn execute(&self, accesor: StorageAccessor) -> Result<String, String>;
}
