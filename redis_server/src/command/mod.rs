use crate::data::storage::service::operator::accessor::StorageAccessor;

pub mod append;
pub mod copy;
pub mod dbsize;
pub mod decrby;
pub mod del;
pub mod exists;
pub mod expire;
pub mod expireat;
pub mod flushdb;
pub mod get;
pub mod getdel;
pub mod getset;
pub mod incrby;
pub mod keys;
pub mod lindex;
pub mod llen;
pub mod mset;
pub mod persist;
pub mod ping;
pub mod rename;
pub mod sadd;
pub mod save;
pub mod scard;
pub mod set;
pub mod sismember;
pub mod sort;
pub mod strlen;
pub mod touch;
pub mod ttl;
pub mod r#type;

pub trait RedisCommand {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String>;
}
