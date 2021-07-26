use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
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
    expiration_value: String,
}

impl RedisCommandExpire {
    pub fn new(key: String, new_value: String) -> RedisCommandExpire {
        RedisCommandExpire {
            key,
            expiration_value: new_value,
        }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command Expire with key: {} and value: {}",
            self.key, self.expiration_value
        ));
        let value = self.expiration_value.parse::<u128>();
        let response = match value {
            Ok(value) => {
                verbose.print(&format!("Value was a number with value: {}", value));
                let response = global_resources
                    .get_storage_accessor()
                    .access(StorageAction::Expire(self.key.clone(), value))?;
                response.get_value().protocol_serialize_to_int()
            }
            Err(_) => {
                verbose.print("Error, value was not a number");
                RedisError::NotANumber.protocol_serialize_to_bulk_string()
            }
        };
        Ok(response)
    }
}
