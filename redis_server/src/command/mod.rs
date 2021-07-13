use crate::data::storage::accessor::StorageAccessor;

pub mod append;
pub mod copy;
pub mod dbsize;
pub mod del;
pub mod exists;
pub mod flushdb;
pub mod get;
pub mod getdel;
pub mod getset;
pub mod lindex;
pub mod ping;
pub mod rename;
pub mod strlen;
pub mod r#type;

pub trait RedisCommand {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String>;
}
