use crate::command::append::RedisCommandAppend;
use crate::command::copy::RedisCommandCopy;
use crate::command::dbsize::RedisCommandDbSize;
use crate::command::decrby::RedisCommandDecrBy;
use crate::command::del::RedisCommandDel;
use crate::command::exists::RedisCommandExists;
use crate::command::flushdb::RedisCommandFlushDb;
use crate::command::get::RedisCommandGet;
use crate::command::getdel::RedisCommandGetDel;
use crate::command::getset::RedisCommandGetSet;
use crate::command::lindex::RedisCommandLindex;
use crate::command::llen::RedisCommandLlen;
use crate::command::ping::RedisCommandPing;
use crate::command::r#type::RedisCommandType;
use crate::command::rename::RedisCommandRename;
use crate::command::strlen::RedisCommandStrlen;
use crate::command::incrby::RedisCommandIncrBy;
use crate::command::RedisCommand;
use std::str::Split;

const TOKEN_SEPARATOR: &str = "\r\n";

pub struct Parser {}
// Simple implementation of parser for our TP
impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&self, packed_command: &[u8]) -> Result<Box<dyn RedisCommand>, String> {
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
        _command_qty: usize,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let command_type = self.parse_string(&mut command_iter)?;
        match command_type.to_uppercase().as_str() {
            "COMMAND" => Ok(Box::new(RedisCommandPing::new())),
            "PING" => Ok(Box::new(RedisCommandPing::new())),
            "INFO" => todo!(),
            "DBSIZE" => Ok(Box::new(RedisCommandDbSize::new())),
            "FLUSHDB" => Ok(Box::new(RedisCommandFlushDb::new())),
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
            "DECRBY" => self.parse_command_decrby(&mut command_iter),
            "INCRBY" => self.parse_command_incrby(&mut command_iter),
            c => Err(format!("Command not implemented: {}", c)),
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
    fn parse_command_exists(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandExists::new(key)))
    }

    fn parse_command_del(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandDel::new(key)))
    }

    fn parse_command_rename(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        let newkey = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandRename::new(key, newkey)))
    }
    fn parse_command_append(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        let new_value = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandAppend::new(key, new_value)))
    }

    fn parse_command_type(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandType::new(key)))
    }

    fn parse_command_copy(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let source_key = self.parse_string(command_iter)?;
        let destination_key = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandCopy::new(source_key, destination_key)))
    }

    fn parse_command_get(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandGet::new(key)))
    }
    fn parse_command_strlen(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandStrlen::new(key)))
    }

    fn parse_command_getset(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        let new_value = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandGetSet::new(key, new_value)))
    }

    fn parse_command_getdel(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandGetDel::new(key)))
    }

    fn parse_command_llen(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandLlen::new(key)))
    }

    fn parse_command_lindex(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        let index = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandLindex::new(key, index)))
    }

    fn parse_command_decrby(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        let value = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandDecrBy::new(key, value)))
    }

    fn parse_command_incrby(
        &self,
        command_iter: &mut Split<&str>,
    ) -> Result<Box<dyn RedisCommand>, String> {
        let key = self.parse_string(command_iter)?;
        let value = self.parse_string(command_iter)?;
        Ok(Box::new(RedisCommandIncrBy::new(key, value)))
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
