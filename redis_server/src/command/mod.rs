use crate::data::storage::accessor::StorageAccessor;

pub mod dbsize;
pub mod del;
pub mod exists;
pub mod flushdb;
pub mod ping;
pub mod rename;
pub mod r#type;

pub trait RedisCommand {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String>;
}
