use crate::data::storage_service::operator_service::accessor::StorageAccessor;

pub mod append;
pub mod copy;
pub mod dbsize;
pub mod decrby;
pub mod del;
pub mod exists;
pub mod expire;
pub mod flushdb;
pub mod get;
pub mod getdel;
pub mod getset;
pub mod incrby;
pub mod lindex;
pub mod llen;
pub mod persist;
pub mod ping;
pub mod rename;
pub mod sadd;
pub mod save;
pub mod sort;
pub mod strlen;
pub mod touch;
pub mod r#type;

pub trait RedisCommand {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String>;
}
