use crate::data::storage_accessor::StorageAccessor;

pub mod dbsize;
pub mod ping;

pub trait RedisCommand {
    fn execute(&self, accesor: StorageAccessor) -> Result<String, String>;
}
