use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::data::storage_service::operator_service::response_message::StorageResult;
use crate::data::storage_service::operator_service::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;

///Returns the element at index index in the list stored at key.
/// The index is zero-based, so 0 means the first element,
/// 1 the second element and so on. Negative indices can be used to designate
/// elements starting at the tail of the list. Here,
/// -1 means the last element, -2 means the penultimate and so forth.
///When the value at key is not a list, an error is returned.
///
/// # Arguments
/// key - String
/// index - String
///
/// # Return value
///Bulk string reply: the requested element, or nil when index is out of range.
pub struct RedisCommandLindex {
    key: String,
    index: String,
}

impl RedisCommandLindex {
    pub fn new(key: String, index: String) -> RedisCommandLindex {
        RedisCommandLindex { key, index }
    }
}

impl RedisCommand for RedisCommandLindex {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = match self.index.parse::<i32>() {
            Ok(index) => {
                let response = accessor.access(StorageAction::Lindex(self.key.clone(), index))?;
                match response.get_value() {
                    StorageResult::String(value) => value.protocol_serialize_to_bulk_string(),
                    value => value.protocol_serialize_to_bulk_string(),
                }
            }
            Err(_) => {
                StorageResult::Error(RedisError::NotANumber).protocol_serialize_to_bulk_string()
            }
        };
        Ok(response)
    }
}
