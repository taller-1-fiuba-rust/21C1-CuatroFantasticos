use crate::redis_response_parser::protocol::array::ArrayResponse;
use crate::redis_response_parser::protocol::error::ErrorResponse;
use crate::redis_response_parser::protocol::integer::IntegerResponse;
use crate::redis_response_parser::protocol::simple_string::SimpleStringResponse;

pub enum RedisResponseType {
    BulkString(BulkStringResponse),
    SimpleString(SimpleStringResponse),
    Error(ErrorResponse),
    Array(ArrayResponse),
    Integer(IntegerResponse),
    Nil,
}
