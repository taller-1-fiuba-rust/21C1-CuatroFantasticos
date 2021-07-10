use crate::data::storage::accessor::StorageAccessor;

pub mod copy;
pub mod dbsize;
pub mod del;
pub mod exists;
pub mod flushdb;
pub mod get;
pub mod ping;
pub mod rename;
pub mod r#type;
pub mod lindex;


pub trait RedisCommand {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String>;
}
