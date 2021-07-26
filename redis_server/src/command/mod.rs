use crate::command::append::RedisCommandAppend;
use crate::command::config_get::RedisCommandConfigGet;
use crate::command::config_set::RedisCommandConfigSet;
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
use crate::command::lpush::RedisCommandLPush;
use crate::command::lpushx::RedisCommandLPushx;
use crate::command::lrange::RedisCommandLRange;
use crate::command::lrem::RedisCommandLRem;
use crate::command::lset::RedisCommandLSet;
use crate::command::mget::RedisCommandMGet;
use crate::command::mset::RedisCommandMSet;
use crate::command::persist::RedisCommandPersist;
use crate::command::ping::RedisCommandPing;
use crate::command::publish::RedisCommandPublish;
use crate::command::r#type::RedisCommandType;
use crate::command::rename::RedisCommandRename;
use crate::command::rpop::RedisCommandRPop;
use crate::command::rpush::RedisCommandRPush;
use crate::command::rpushx::RedisCommandRPushx;
use crate::command::sadd::RedisCommandSAdd;
use crate::command::save::RedisCommandSave;
use crate::command::scard::RedisCommandScard;
use crate::command::set::RedisCommandSet;
use crate::command::sismember::RedisCommandSismember;
use crate::command::smembers::RedisCommandSmembers;
use crate::command::sort::RedisCommandSort;
use crate::command::srem::RedisCommandSrem;
use crate::command::strlen::RedisCommandStrlen;
use crate::command::subscribe::RedisCommandSubscribe;
use crate::command::touch::RedisCommandTouch;
use crate::command::ttl::RedisCommandTtl;
use crate::global_resources::GlobalResources;

pub mod append;
pub mod config_get;
pub mod config_set;
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
pub mod lpush;
pub mod lpushx;
pub mod lrange;
pub mod lrem;
pub mod lset;
pub mod mget;
pub mod mset;
pub mod persist;
pub mod ping;
pub mod publish;
pub mod rename;
pub mod rpop;
pub mod rpush;
pub mod rpushx;
pub mod sadd;
pub mod save;
pub mod scard;
pub mod set;
pub mod sismember;
pub mod smembers;
pub mod sort;
pub mod srem;
pub mod strlen;
pub mod subscribe;
pub mod touch;
pub mod ttl;
pub mod r#type;

pub enum RedisCommand {
    Append(RedisCommandAppend),
    ConfigGet(RedisCommandConfigGet),
    ConfigSet(RedisCommandConfigSet),
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
    Lpush(RedisCommandLPush),
    Lpushx(RedisCommandLPushx),
    Lrange(RedisCommandLRange),
    Lrem(RedisCommandLRem),
    Lset(RedisCommandLSet),
    Mget(RedisCommandMGet),
    Mset(RedisCommandMSet),
    Persist(RedisCommandPersist),
    Ping(RedisCommandPing),
    Publish(RedisCommandPublish),
    Rename(RedisCommandRename),
    Rpop(RedisCommandRPop),
    Rpush(RedisCommandRPush),
    Rpushx(RedisCommandRPushx),
    Sadd(RedisCommandSAdd),
    Save(RedisCommandSave),
    Scard(RedisCommandScard),
    Srem(RedisCommandSrem),
    Set(RedisCommandSet),
    Sismember(RedisCommandSismember),
    Smembers(RedisCommandSmembers),
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
            RedisCommand::ConfigGet(c) => c.execute(global_resources),
            RedisCommand::ConfigSet(c) => c.execute(global_resources),
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
            RedisCommand::Lpop(c) => c.execute(global_resources),
            RedisCommand::Lpush(c) => c.execute(global_resources),
            RedisCommand::Lpushx(c) => c.execute(global_resources),
            RedisCommand::Lrange(c) => c.execute(global_resources),
            RedisCommand::Lrem(c) => c.execute(global_resources),
            RedisCommand::Lset(c) => c.execute(global_resources),
            RedisCommand::Mget(c) => c.execute(global_resources),
            RedisCommand::Mset(c) => c.execute(global_resources),
            RedisCommand::Persist(c) => c.execute(global_resources),
            RedisCommand::Ping(c) => c.execute(global_resources),
            RedisCommand::Publish(c) => c.execute(global_resources),
            RedisCommand::Rename(c) => c.execute(global_resources),
            RedisCommand::Rpop(c) => c.execute(global_resources),
            RedisCommand::Rpush(c) => c.execute(global_resources),
            RedisCommand::Rpushx(c) => c.execute(global_resources),
            RedisCommand::Sadd(c) => c.execute(global_resources),
            RedisCommand::Save(c) => c.execute(global_resources),
            RedisCommand::Scard(c) => c.execute(global_resources),
            RedisCommand::Set(c) => c.execute(global_resources),
            RedisCommand::Sismember(c) => c.execute(global_resources),
            RedisCommand::Smembers(c) => c.execute(global_resources),
            RedisCommand::Sort(c) => c.execute(global_resources),
            RedisCommand::Strlen(c) => c.execute(global_resources),
            RedisCommand::Srem(c) => c.execute(global_resources),
            RedisCommand::Subscribe(c) => c.execute(global_resources),
            RedisCommand::Touch(c) => c.execute(global_resources),
            RedisCommand::Ttl(c) => c.execute(global_resources),
            RedisCommand::Type(c) => c.execute(global_resources),
        }
    }
}
