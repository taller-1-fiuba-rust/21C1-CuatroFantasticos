use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;
///Set a timeout on key. After the timeout has expired,
/// the key will automatically be deleted. A key with an associated
/// timeout is often said to be volatile in Redis terminology.
/// # Arguments
/// *  key - String,
/// *  new_value - String,
///
///# Return value
///Integer reply, specifically:
/// * 1 if the timeout was set.
/// * 0 if key does not exist.
///
pub struct RedisCommandExpire {
    key: String,
    new_value: String,
}

impl RedisCommandExpire {
    pub fn new(key: String, new_value: String) -> RedisCommandExpire {
        RedisCommandExpire { key, new_value }
    }
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let value = self.new_value.parse::<u128>();
        let response = match value {
            Ok(value) => {
                let response = accessor.access(StorageAction::Expire(self.key.clone(), value))?;
                response.get_value().protocol_serialize_to_int()
            }
            Err(_) => RedisError::NotANumber.protocol_serialize_to_bulk_string(),
        };
        Ok(response)
    }
}
