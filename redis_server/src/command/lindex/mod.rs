use crate::command::RedisCommand;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;

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
