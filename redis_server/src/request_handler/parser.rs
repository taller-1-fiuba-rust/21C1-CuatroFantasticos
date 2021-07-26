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
use crate::command::RedisCommand;
use std::str::Split;

const TOKEN_SEPARATOR: &str = "\r\n";

pub struct Parser {}
// Simple implementation of parser for our TP
impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&self, packed_command: &[u8]) -> Result<RedisCommand, String> {
        let mut command_iter = std::str::from_utf8(packed_command)
            .map_err(|_| "Not an utf-8 string".to_string())?
            .split(TOKEN_SEPARATOR);
        let bulk_len_token = command_iter.next().ok_or("Empty command")?;
        let argc = self.parse_bulk_len(bulk_len_token)?;
        self.parse_command(command_iter, argc)
    }

    fn parse_command(
        &self,
        mut command_iter: Split<&str>,
        command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let command_type = self.parse_string(&mut command_iter)?;
        match command_type.to_uppercase().as_str() {
            "COMMAND" => Ok(RedisCommand::Ping(RedisCommandPing::new())),
            "PING" => Ok(RedisCommand::Ping(RedisCommandPing::new())),
            "INFO" => todo!(),
            "KEYS" => self.parse_command_keys(&mut command_iter),
            "DBSIZE" => Ok(RedisCommand::Dbsize(RedisCommandDbSize::new())),
            "FLUSHDB" => Ok(RedisCommand::FlushDb(RedisCommandFlushDb::new())),
            "TYPE" => self.parse_command_type(&mut command_iter),
            "EXISTS" => self.parse_command_exists(&mut command_iter),
            "RENAME" => self.parse_command_rename(&mut command_iter),
            "DEL" => self.parse_command_del(&mut command_iter),
            "COPY" => self.parse_command_copy(&mut command_iter),
            "GET" => self.parse_command_get(&mut command_iter),
            "APPEND" => self.parse_command_append(&mut command_iter),
            "GETDEL" => self.parse_command_getdel(&mut command_iter),
            "GETSET" => self.parse_command_getset(&mut command_iter),
            "STRLEN" => self.parse_command_strlen(&mut command_iter),
            "LLEN" => self.parse_command_llen(&mut command_iter),
            "LINDEX" => self.parse_command_lindex(&mut command_iter),
            "SORT" => self.parse_command_sort(&mut command_iter),
            "DECRBY" => self.parse_command_decrby(&mut command_iter),
            "INCRBY" => self.parse_command_incrby(&mut command_iter),
            "TOUCH" => self.parse_command_touch(&mut command_iter),
            "SADD" => self.parse_command_sadd(&mut command_iter, command_qty),
            "LPUSH" => self.parse_command_lpush(&mut command_iter, command_qty),
            "RPUSH" => self.parse_command_rpush(&mut command_iter, command_qty),
            "RPUSHX" => self.parse_command_rpushx(&mut command_iter, command_qty),
            "LPUSHX" => self.parse_command_lpushx(&mut command_iter, command_qty),
            "TTL" => self.parse_command_ttl(&mut command_iter),
            "PERSIST" => self.parse_command_persist(&mut command_iter),
            "SAVE" => Ok(RedisCommand::Save(RedisCommandSave::new())),
            "EXPIRE" => self.parse_command_expire(&mut command_iter),
            "EXPIREAT" => self.parse_command_expireat(&mut command_iter),
            "SCARD" => self.parse_command_scard(&mut command_iter),
            "SISMEMBER" => self.parse_command_sismember(&mut command_iter),
            "SMEMBERS" => self.parse_command_smembers(&mut command_iter),
            "SET" => self.parse_command_set(&mut command_iter),
            "SREM" => self.parse_command_srem(&mut command_iter, command_qty),
            "MSET" => self.parse_command_mset(&mut command_iter, command_qty),
            "MGET" => self.parse_command_mget(&mut command_iter, command_qty),
            "SUBSCRIBE" => self.parse_command_subscribe(&mut command_iter, command_qty),
            "PUBLISH" => self.parse_command_publish(&mut command_iter, command_qty),
            "LPOP" => self.parse_command_lpop(&mut command_iter, command_qty),
            "RPOP" => self.parse_command_rpop(&mut command_iter, command_qty),
            "LSET" => self.parse_command_lset(&mut command_iter),
            "CONFIG" => self.parse_command_config(&mut command_iter),
            "LRANGE" => self.parse_command_lrange(&mut command_iter),
            "LREM" => self.parse_command_lrem(&mut command_iter),
            _ => Err(format!(
                "-Unknown or disabled command '{}'\r\n",
                command_type
            )),
        }
    }

    fn parse_bulk_len(&self, command_part: &str) -> Result<usize, String> {
        if &command_part[..1] != "*" {
            return Err("Not a bulk len token".to_string());
        }
        let len = (&command_part[1..])
            .parse::<usize>()
            .map_err(|_| "Not a numeric length".to_string())?;
        Ok(len)
    }

    fn parse_string(&self, command_iter: &mut Split<&str>) -> Result<String, String> {
        let command_part = command_iter.next().ok_or("End of input")?;
        if &command_part[..1] != "$" {
            return Err("Not a string token".to_string());
        }
        let command_part = command_iter.next().ok_or("End of input")?;
        Ok(command_part.to_string())
    }
}

impl Parser {
    fn parse_command_exists(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Exists(RedisCommandExists::new(key)))
    }
    fn parse_command_keys(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Keys(RedisCommandKeys::new(key)))
    }
    fn parse_command_del(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Del(RedisCommandDel::new(key)))
    }
    fn parse_command_rename(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let newkey = self.parse_string(command_iter)?;
        Ok(RedisCommand::Rename(RedisCommandRename::new(key, newkey)))
    }
    fn parse_command_append(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let new_value = self.parse_string(command_iter)?;
        Ok(RedisCommand::Append(RedisCommandAppend::new(
            key, new_value,
        )))
    }
    fn parse_command_type(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Type(RedisCommandType::new(key)))
    }
    fn parse_command_copy(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let source_key = self.parse_string(command_iter)?;
        let destination_key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Copy(RedisCommandCopy::new(
            source_key,
            destination_key,
        )))
    }
    fn parse_command_get(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Get(RedisCommandGet::new(key)))
    }
    fn parse_command_strlen(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Strlen(RedisCommandStrlen::new(key)))
    }
    fn parse_command_scard(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Scard(RedisCommandScard::new(key)))
    }
    fn parse_command_getset(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let new_value = self.parse_string(command_iter)?;
        Ok(RedisCommand::GetSet(RedisCommandGetSet::new(
            key, new_value,
        )))
    }
    fn parse_command_getdel(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::GetDel(RedisCommandGetDel::new(key)))
    }
    fn parse_command_llen(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Llen(RedisCommandLlen::new(key)))
    }
    fn parse_command_lindex(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let index = self.parse_string(command_iter)?;
        Ok(RedisCommand::Lindex(RedisCommandLindex::new(key, index)))
    }
    fn parse_command_sort(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Sort(RedisCommandSort::new(key)))
    }
    fn parse_command_decrby(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let value = self.parse_string(command_iter)?;
        Ok(RedisCommand::DecrBy(RedisCommandDecrBy::new(key, value)))
    }
    fn parse_command_incrby(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let value = self.parse_string(command_iter)?;
        Ok(RedisCommand::IncrBy(RedisCommandIncrBy::new(key, value)))
    }
    fn parse_command_touch(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Touch(RedisCommandTouch::new(key)))
    }
    fn parse_command_persist(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Persist(RedisCommandPersist::new(key)))
    }
    fn parse_command_sadd(
        &self,
        command_iter: &mut Split<&str>,
        command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let mut members = Vec::<String>::new();
        for _ in 1..(command_qty - 1) {
            let new_member = self.parse_string(command_iter)?;
            members.push(new_member);
        }
        Ok(RedisCommand::Sadd(RedisCommandSAdd::new(key, members)))
    }
    fn parse_command_lpush(
        &self,
        command_iter: &mut Split<&str>,
        command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let mut members = Vec::<String>::new();
        for _ in 1..(command_qty - 1) {
            let new_member = self.parse_string(command_iter)?;
            members.push(new_member);
        }
        Ok(RedisCommand::Lpush(RedisCommandLPush::new(key, members)))
    }
    fn parse_command_lpushx(
        &self,
        command_iter: &mut Split<&str>,
        command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let mut members = Vec::<String>::new();
        for _ in 1..(command_qty - 1) {
            let new_member = self.parse_string(command_iter)?;
            members.push(new_member);
        }
        Ok(RedisCommand::Lpushx(RedisCommandLPushx::new(key, members)))
    }
    fn parse_command_rpush(
        &self,
        command_iter: &mut Split<&str>,
        command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let mut members = Vec::<String>::new();
        for _ in 1..(command_qty - 1) {
            let new_member = self.parse_string(command_iter)?;
            members.push(new_member);
        }
        Ok(RedisCommand::Rpush(RedisCommandRPush::new(key, members)))
    }
    fn parse_command_rpushx(
        &self,
        command_iter: &mut Split<&str>,
        command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let mut members = Vec::<String>::new();
        for _ in 1..(command_qty - 1) {
            let new_member = self.parse_string(command_iter)?;
            members.push(new_member);
        }
        Ok(RedisCommand::Rpushx(RedisCommandRPushx::new(key, members)))
    }
    fn parse_command_srem(
        &self,
        command_iter: &mut Split<&str>,
        command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let mut members = Vec::<String>::new();
        for _ in 1..(command_qty - 1) {
            let new_member = self.parse_string(command_iter)?;
            members.push(new_member);
        }
        Ok(RedisCommand::Srem(RedisCommandSrem::new(key, members)))
    }
    fn parse_command_mset(
        &self,
        command_iter: &mut Split<&str>,
        command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let mut keys = Vec::<String>::new();
        let mut values = Vec::<String>::new();
        for _ in 0..(command_qty / 2) {
            let new_key = self.parse_string(command_iter)?;
            let new_value = self.parse_string(command_iter)?;
            keys.push(new_key);
            values.push(new_value);
        }
        Ok(RedisCommand::Mset(RedisCommandMSet::new(keys, values)))
    }
    fn parse_command_expire(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let value = self.parse_string(command_iter)?;
        Ok(RedisCommand::Expire(RedisCommandExpire::new(key, value)))
    }
    fn parse_command_smembers(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Smembers(RedisCommandSmembers::new(key)))
    }
    fn parse_command_expireat(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let value = self.parse_string(command_iter)?;
        Ok(RedisCommand::ExpireAt(RedisCommandExpireAt::new(
            key, value,
        )))
    }
    fn parse_command_ttl(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        Ok(RedisCommand::Ttl(RedisCommandTtl::new(key)))
    }
    fn parse_command_sismember(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let member = self.parse_string(command_iter)?;
        Ok(RedisCommand::Sismember(RedisCommandSismember::new(
            key, member,
        )))
    }
    fn parse_command_set(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let value = self.parse_string(command_iter)?;
        Ok(RedisCommand::Set(RedisCommandSet::new(key, value)))
    }
    fn parse_command_mget(
        &self,
        command_iter: &mut Split<&str>,
        command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let mut keys = Vec::<String>::new();
        for _ in 1..command_qty {
            let new_key = self.parse_string(command_iter)?;
            keys.push(new_key);
        }
        Ok(RedisCommand::Mget(RedisCommandMGet::new(keys)))
    }
    fn parse_command_subscribe(
        &self,
        command_iter: &mut Split<&str>,
        command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let mut channels = Vec::<String>::new();
        for _ in 1..command_qty {
            let new_channel = self.parse_string(command_iter)?;
            channels.push(new_channel);
        }
        Ok(RedisCommand::Subscribe(RedisCommandSubscribe::new(
            channels,
        )))
    }

    fn parse_command_publish(
        &self,
        command_iter: &mut Split<&str>,
        _command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let channel = self.parse_string(command_iter)?;
        let message = self.parse_string(command_iter)?;
        Ok(RedisCommand::Publish(RedisCommandPublish::new(
            channel, message,
        )))
    }

    fn parse_command_lpop(
        &self,
        command_iter: &mut Split<&str>,
        command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let times;
        if command_qty == 3 {
            times = self.parse_string(command_iter)?;
        } else {
            times = String::from("1");
        }
        Ok(RedisCommand::Lpop(RedisCommandLPop::new(key, times)))
    }
    fn parse_command_rpop(
        &self,
        command_iter: &mut Split<&str>,
        command_qty: usize,
    ) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let times;
        if command_qty == 3 {
            times = self.parse_string(command_iter)?;
        } else {
            times = String::from("1");
        }
        Ok(RedisCommand::Rpop(RedisCommandRPop::new(key, times)))
    }
    fn parse_command_lset(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let index = self.parse_string(command_iter)?;
        let value = self.parse_string(command_iter)?;
        Ok(RedisCommand::Lset(RedisCommandLSet::new(key, index, value)))
    }
    fn parse_command_config(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        match self.parse_string(command_iter)?.to_uppercase().as_str() {
            "GET" => {
                let key = self.parse_string(command_iter)?;
                Ok(RedisCommand::ConfigGet(RedisCommandConfigGet::new(key)))
            }
            "SET" => {
                let key = self.parse_string(command_iter)?;
                let value = self.parse_string(command_iter)?;
                Ok(RedisCommand::ConfigSet(RedisCommandConfigSet::new(
                    key, value,
                )))
            }
            _ => todo!(),
        }
    }
    fn parse_command_lrange(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let start = self.parse_string(command_iter)?;
        let stop = self.parse_string(command_iter)?;
        Ok(RedisCommand::Lrange(RedisCommandLRange::new(
            key, start, stop,
        )))
    }
    fn parse_command_lrem(&self, command_iter: &mut Split<&str>) -> Result<RedisCommand, String> {
        let key = self.parse_string(command_iter)?;
        let count = self.parse_string(command_iter)?;
        let element = self.parse_string(command_iter)?;
        Ok(RedisCommand::Lrem(RedisCommandLRem::new(
            key, count, element,
        )))
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
