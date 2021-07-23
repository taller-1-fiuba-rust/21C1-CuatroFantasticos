use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::data::storage_service::operator_service::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;
///EXPIREAT has the same effect and semantic as EXPIRE,
/// but instead of specifying the number of seconds representing the TTL
/// (time to live), it takes an absolute Unix timestamp (seconds since January 1, 1970)
/// . A timestamp in the past will delete the key immediately.
///
/// # Arguments
/// *  key - String,
/// *  new_value - String,
///
///# Return value
///Integer reply, specifically:
/// * 1 if the timeout was set.
/// * 0 if key does not exist.

pub struct RedisCommandExpireAt {
    key: String,
    new_value: String,
}

impl RedisCommandExpireAt {
    pub fn new(key: String, new_value: String) -> RedisCommandExpireAt {
        RedisCommandExpireAt { key, new_value }
    }
}

impl RedisCommand for RedisCommandExpireAt {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let value = self.new_value.parse::<u128>();
        let response = match value {
            Ok(value) => {
                let response = accessor.access(StorageAction::ExpireAt(self.key.clone(), value))?;
                response.get_value().protocol_serialize_to_int()
            }
            Err(_) => RedisError::NotANumber.protocol_serialize_to_bulk_string(),
        };
        Ok(response)
    }
}
