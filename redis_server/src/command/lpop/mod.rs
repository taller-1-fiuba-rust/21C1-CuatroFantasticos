use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandLPop {
    key: String,
    times: String,
}

impl RedisCommandLPop {
    pub fn new(key: String, times: String) -> RedisCommandLPop {
        RedisCommandLPop { key, times }
    }
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let times = self.times.parse::<i32>();
        let response = match times {
            Ok(value) => {
                let response = accessor.access(StorageAction::LPop(self.key.clone(), value))?;
                response.get_value().protocol_serialize_to_simple_string()
            }
            Err(_) => RedisError::NotANumber.protocol_serialize_to_bulk_string(),
        };
        Ok(response)
    }
}
