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
use crate::command::lpop::RedisCommandLPop;
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
use crate::command::srem::RedisCommandSrem;
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
pub mod lpop;
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
pub mod srem;
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
    Lpop(RedisCommandLPop),
    Mget(RedisCommandMGet),
    Mset(RedisCommandMSet),
    Persist(RedisCommandPersist),
    Ping(RedisCommandPing),
    Rename(RedisCommandRename),
    Sadd(RedisCommandSAdd),
    Save(RedisCommandSave),
    Scard(RedisCommandScard),
    Srem(RedisCommandSrem),
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
        let accessor = global_resources.get_storage_accessor();
        match self {
            RedisCommand::Append(c) => c.execute(accessor),
            RedisCommand::Copy(c) => c.execute(accessor),
            RedisCommand::Dbsize(c) => c.execute(accessor),
            RedisCommand::DecrBy(c) => c.execute(accessor),
            RedisCommand::Del(c) => c.execute(accessor),
            RedisCommand::Exists(c) => c.execute(accessor),
            RedisCommand::Expire(c) => c.execute(accessor),
            RedisCommand::ExpireAt(c) => c.execute(accessor),
            RedisCommand::FlushDb(c) => c.execute(accessor),
            RedisCommand::Get(c) => c.execute(accessor),
            RedisCommand::GetDel(c) => c.execute(accessor),
            RedisCommand::GetSet(c) => c.execute(accessor),
            RedisCommand::IncrBy(c) => c.execute(accessor),
            RedisCommand::Keys(c) => c.execute(accessor),
            RedisCommand::Lindex(c) => c.execute(accessor),
            RedisCommand::Llen(c) => c.execute(accessor),
            RedisCommand::Lpop(c) => c.execute(global_resources),
            RedisCommand::Mget(c) => c.execute(accessor),
            RedisCommand::Mset(c) => c.execute(accessor),
            RedisCommand::Persist(c) => c.execute(accessor),
            RedisCommand::Ping(c) => c.execute(accessor),
            RedisCommand::Rename(c) => c.execute(accessor),
            RedisCommand::Sadd(c) => c.execute(accessor),
            RedisCommand::Save(c) => c.execute(accessor),
            RedisCommand::Scard(c) => c.execute(accessor),
            RedisCommand::Set(c) => c.execute(accessor),
            RedisCommand::Sismember(c) => c.execute(accessor),
            RedisCommand::Sort(c) => c.execute(accessor),
            RedisCommand::Srem(c) => c.execute(global_resources),
            RedisCommand::Strlen(c) => c.execute(accessor),
            RedisCommand::Subscribe(c) => c.execute(global_resources),
            RedisCommand::Touch(c) => c.execute(accessor),
            RedisCommand::Ttl(c) => c.execute(accessor),
            RedisCommand::Type(c) => c.execute(accessor),
        }
    }
}
