use crate::command::append::RedisCommandAppend;
use crate::command::copy::RedisCommandCopy;
use crate::command::dbsize::RedisCommandDbSize;
use crate::command::decrby::RedisCommandDecrBy;
use crate::command::del::RedisCommandDel;
use crate::command::exists::RedisCommandExists;
use crate::command::expire::RedisCommandExpire;
use crate::command::expireat::RedisCommandExpireAt;
use crate::command::flushdb::RedisCommandFlushDb;
use crate::command::get::RedisCommandGet;
use crate::command::getdel::RedisCommandGetDel;
use crate::command::getset::RedisCommandGetSet;
use crate::command::incrby::RedisCommandIncrBy;
use crate::command::keys::RedisCommandKeys;
use crate::command::lindex::RedisCommandLindex;
use crate::command::llen::RedisCommandLlen;
use crate::command::mget::RedisCommandMGet;
use crate::command::mset::RedisCommandMSet;
use crate::command::persist::RedisCommandPersist;
use crate::command::ping::RedisCommandPing;
use crate::command::r#type::RedisCommandType;
use crate::command::rename::RedisCommandRename;
use crate::command::sadd::RedisCommandSAdd;
use crate::command::save::RedisCommandSave;
use crate::command::scard::RedisCommandScard;
use crate::command::set::RedisCommandSet;
use crate::command::sismember::RedisCommandSismember;
use crate::command::sort::RedisCommandSort;
use crate::command::strlen::RedisCommandStrlen;
use crate::command::subscribe::RedisCommandSubscribe;
use crate::command::touch::RedisCommandTouch;
use crate::command::ttl::RedisCommandTtl;
use crate::global_resources::GlobalResources;

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
pub mod mget;
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
pub mod subscribe;
pub mod touch;
pub mod ttl;
pub mod r#type;

pub enum RedisCommand {
    Append(RedisCommandAppend),
    Copy(RedisCommandCopy),
    Dbsize(RedisCommandDbSize),
    DecrBy(RedisCommandDecrBy),
    Del(RedisCommandDel),
    Exists(RedisCommandExists),
    Expire(RedisCommandExpire),
    ExpireAt(RedisCommandExpireAt),
    FlushDb(RedisCommandFlushDb),
    Get(RedisCommandGet),
    GetDel(RedisCommandGetDel),
    GetSet(RedisCommandGetSet),
    IncrBy(RedisCommandIncrBy),
    Keys(RedisCommandKeys),
    Lindex(RedisCommandLindex),
    Llen(RedisCommandLlen),
    Mget(RedisCommandMGet),
    Mset(RedisCommandMSet),
    Persist(RedisCommandPersist),
    Ping(RedisCommandPing),
    Rename(RedisCommandRename),
    Sadd(RedisCommandSAdd),
    Save(RedisCommandSave),
    Scard(RedisCommandScard),
    Set(RedisCommandSet),
    Sismember(RedisCommandSismember),
    Sort(RedisCommandSort),
    Strlen(RedisCommandStrlen),
    Subscribe(RedisCommandSubscribe),
    Touch(RedisCommandTouch),
    Ttl(RedisCommandTtl),
    Type(RedisCommandType),
}
impl RedisCommand {
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        match self {
            RedisCommand::Append(c) => c.execute(global_resources),
            RedisCommand::Copy(c) => c.execute(global_resources),
            RedisCommand::Dbsize(c) => c.execute(global_resources),
            RedisCommand::DecrBy(c) => c.execute(global_resources),
            RedisCommand::Del(c) => c.execute(global_resources),
            RedisCommand::Exists(c) => c.execute(global_resources),
            RedisCommand::Expire(c) => c.execute(global_resources),
            RedisCommand::ExpireAt(c) => c.execute(global_resources),
            RedisCommand::FlushDb(c) => c.execute(global_resources),
            RedisCommand::Get(c) => c.execute(global_resources),
            RedisCommand::GetDel(c) => c.execute(global_resources),
            RedisCommand::GetSet(c) => c.execute(global_resources),
            RedisCommand::IncrBy(c) => c.execute(global_resources),
            RedisCommand::Keys(c) => c.execute(global_resources),
            RedisCommand::Lindex(c) => c.execute(global_resources),
            RedisCommand::Llen(c) => c.execute(global_resources),
            RedisCommand::Mget(c) => c.execute(global_resources),
            RedisCommand::Mset(c) => c.execute(global_resources),
            RedisCommand::Persist(c) => c.execute(global_resources),
            RedisCommand::Ping(c) => c.execute(global_resources),
            RedisCommand::Rename(c) => c.execute(global_resources),
            RedisCommand::Sadd(c) => c.execute(global_resources),
            RedisCommand::Save(c) => c.execute(global_resources),
            RedisCommand::Scard(c) => c.execute(global_resources),
            RedisCommand::Set(c) => c.execute(global_resources),
            RedisCommand::Sismember(c) => c.execute(global_resources),
            RedisCommand::Sort(c) => c.execute(global_resources),
            RedisCommand::Strlen(c) => c.execute(global_resources),
            RedisCommand::Subscribe(c) => c.execute(global_resources),
            RedisCommand::Touch(c) => c.execute(global_resources),
            RedisCommand::Ttl(c) => c.execute(global_resources),
            RedisCommand::Type(c) => c.execute(global_resources),
        }
    }
}
