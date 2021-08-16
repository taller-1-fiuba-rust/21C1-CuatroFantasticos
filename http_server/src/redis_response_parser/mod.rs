mod protocol;
mod response_type;

use crate::redis_response_parser::protocol::array::ArrayResponse;
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
        let response = self.parse_response_by_type(&mut response_iter);
        response.map(|v| v.to_client_string())
    }
    pub fn parse_response_by_type(
        &self,
        response_iter: &mut Split<&str>,
    ) -> Result<Box<dyn DisplayRedisResponse>, RedisResponseParserError> {
        let response_type = response_iter
            .next()
            .ok_or(RedisResponseParserError::EmptyCommand)?;
        if response_type == "$-1" {
            return Ok(Box::new(NilResponse::new()));
        }
        match &response_type[..1] {
            "*" => {
                let len = (&response_type[1..])
                    .parse()
                    .map_err(|_| RedisResponseParserError::NotAValidServerResponse)?;
                let response = self.parse_array(response_iter, len)?;
                Ok(Box::new(response))
            }
            "$" => {
                let response = self.parse_bulk_string(response_iter)?;
                Ok(Box::new(response))
            }
            ":" => {
                let len = (&response_type[1..])
                    .parse()
                    .map_err(|_| RedisResponseParserError::NotAValidServerResponse)?;
                let response = self.parse_integer(len)?;
                Ok(Box::new(response))
            }
            "-" => {
                let error = &response_type[1..];
                let response = self.parse_error(error.to_owned())?;
                Ok(Box::new(response))
            }
            "+" => {
                let string = &response_type[1..];
                let response = self.parse_simple_string(string.to_owned())?;
                Ok(Box::new(response))
            }
            _ => Err(RedisResponseParserError::NotAValidServerResponse),
        }
    }
    pub fn parse_bulk_string(
        &self,
        response_iter: &mut Split<&str>,
    ) -> Result<BulkStringResponse, RedisResponseParserError> {
        let value = response_iter
            .next()
            .ok_or(RedisResponseParserError::NotAValidServerResponse)?;
        let response = BulkStringResponse::new(value.to_owned());
        Ok(response)
    }
    pub fn parse_simple_string(
        &self,
        string: String,
    ) -> Result<SimpleStringResponse, RedisResponseParserError> {
        let response = SimpleStringResponse::new(string);
        Ok(response)
    }
    pub fn parse_error(&self, string: String) -> Result<ErrorResponse, RedisResponseParserError> {
        let response = ErrorResponse::new(string);
        Ok(response)
    }
    pub fn parse_integer(&self, value: i64) -> Result<IntegerResponse, RedisResponseParserError> {
        let response = IntegerResponse::new(value);
        Ok(response)
    }
    pub fn parse_nil(&self) -> Result<NilResponse, RedisResponseParserError> {
        let response = NilResponse::new();
        Ok(response)
    }
    pub fn parse_array(
        &self,
        response_iter: &mut Split<&str>,
        len: usize,
    ) -> Result<ArrayResponse, RedisResponseParserError> {
        let mut members = Vec::new();
        for _ in 1..len {
            let member = self.parse_response_by_type(response_iter)?;
            members.push(member);
        }
        Ok(ArrayResponse::new(members))
    }
}

impl Default for RedisResponseParser {
    fn default() -> Self {
        RedisResponseParser {}
    }
}
