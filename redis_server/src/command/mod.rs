use crate::data::storage::accessor::StorageAccessor;

pub mod copy;
pub mod dbsize;
pub mod del;
pub mod exists;
pub mod flushdb;
pub mod ping;
pub mod rename;
pub mod r#type;

pub trait RedisCommand {
    fn execute(&self, accesor: StorageAccessor) -> Result<String, String>;
}
