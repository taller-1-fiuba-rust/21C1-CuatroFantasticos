mod protocol;
mod response_type;

use crate::redis_response_parser::protocol::bulk_string::BulkStringResponse;
use crate::redis_response_parser::protocol::error::ErrorResponse;
use crate::redis_response_parser::protocol::integer::IntegerResponse;
use crate::redis_response_parser::protocol::nil::NilResponse;
use crate::redis_response_parser::protocol::simple_string::SimpleStringResponse;
use crate::redis_response_parser::protocol::DisplayRedisResponse;
use crate::redis_response_parser::response_type::RedisResponseType;
use std::str::Split;

const TOKEN_SEPARATOR: &str = "\r\n";

pub struct RedisResponseParser {}

pub enum RedisResponseParserError {
    NotUtf8,
    NotAValidServerResponse,
    WrongArgNumber,
    EmptyCommand,
}

impl RedisResponseParser {
    pub fn new() -> Self {
        RedisResponseParser {}
    }
    pub fn parse(&self, packed_command: &[u8]) -> Result<String, RedisResponseParserError> {
        let mut response_iter = std::str::from_utf8(packed_command)
            .map_err(|_| RedisResponseParserError::NotUtf8)?
            .split(TOKEN_SEPARATOR);
        self.parse_response(response_iter)
    }
    pub fn parse_response(
        &self,
        mut response_iter: Split<&str>,
    ) -> Result<String, RedisResponseParserError> {
        let command_type = self.parse_response_type(&mut response_iter)?;
        match command_type {
            RedisResponseType::BulkString => self.parse_bulk_string(&mut response_iter),
            RedisResponseType::SimpleString(string) => {
                let response = SimpleStringResponse::new(string);
                Ok(response.to_client_string())
            }
            RedisResponseType::Error(string) => {
                let response = ErrorResponse::new(string);
                Ok(response.to_client_string())
            }
            RedisResponseType::Array(size) => {
                todo!()
            }
            RedisResponseType::Integer(value) => {
                let response = IntegerResponse::new(value);
                Ok(response.to_client_string())
            }
            RedisResponseType::Nil => {
                let response = NilResponse::new();
                Ok(response.to_client_string())
            }
        }
    }
    pub fn parse_response_type(
        &self,
        response_iter: &mut Split<&str>,
    ) -> Result<RedisResponseType, RedisResponseParserError> {
        let response_type = response_iter
            .next()
            .ok_or(RedisResponseParserError::EmptyCommand)?;
        match &response_type[..1] {
            "*" => {
                let size = (&response_type[1..])
                    .parse()
                    .map_err(|_| RedisResponseParserError::NotAValidServerResponse)?;
                Ok(RedisResponseType::Array(size))
            }
            "$" => Ok(RedisResponseType::BulkString),
            ":" => {
                let len = (&response_type[1..])
                    .parse()
                    .map_err(|_| RedisResponseParserError::NotAValidServerResponse)?;
                Ok(RedisResponseType::Integer(len))
            }
            "-" => {
                let error = &response_type[1..];
                Ok(RedisResponseType::Error(error.to_owned()))
            }
            "+" => {
                let string = &response_type[1..];
                Ok(RedisResponseType::SimpleString(string.to_owned()))
            }
            _ => Err(RedisResponseParserError::NotAValidServerResponse),
        }
    }
    pub fn parse_bulk_string(
        &self,
        response_iter: &mut Split<&str>,
    ) -> Result<String, RedisResponseParserError> {
        let value = response_iter
            .next()
            .ok_or(RedisResponseParserError::NotAValidServerResponse)?;
        let response = BulkStringResponse::new(value.to_owned());
        Ok(response.to_client_string())
    }
}

impl Default for RedisResponseParser {
    fn default() -> Self {
        RedisResponseParser {}
    }
}
