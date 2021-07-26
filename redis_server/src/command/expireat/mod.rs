use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
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
    expiration_value: String,
}

impl RedisCommandExpireAt {
    pub fn new(key: String, new_value: String) -> RedisCommandExpireAt {
        RedisCommandExpireAt {
            key,
            expiration_value: new_value,
        }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!(
            "Executing command ExpireAt with key: {} and value: {}",
            self.key, self.expiration_value
        ));
        let value = self.expiration_value.parse::<u128>();
        let response = match value {
            Ok(value) => {
                verbose.print(&format!(
                    "Value of argument was a number with value: {}",
                    value
                ));
                let response = global_resources
                    .get_storage_accessor()
                    .access(StorageAction::ExpireAt(self.key.clone(), value))?;
                response.get_value().protocol_serialize_to_int()
            }
            Err(_) => {
                verbose.print("Error, value of argument was not a number");
                RedisError::NotANumber.protocol_serialize_to_bulk_string()
            }
        };
        verbose.print("Finalizing execution of command ExpireAt");
        Ok(response)
    }
}
