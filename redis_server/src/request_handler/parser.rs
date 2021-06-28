use crate::command::dbsize::RedisCommandDbSize;
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
        match command_type.as_str() {
            "INFO" => self.create_command_type_info(),
            "DBSIZE" => self.create_command_type_dbsize(),
            _ => Err("Command not implemented".to_string()),
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
    fn create_command_type_info(&self) -> Result<Box<dyn RedisCommand>, String> {
        todo!()
    }

    fn create_command_type_dbsize(&self) -> Result<Box<dyn RedisCommand>, String> {
        Ok(Box::new(RedisCommandDbSize::new()))
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
