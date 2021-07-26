use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Returns all keys matching pattern.
/// While the time complexity for this operation is O(N),
/// the constant times are fairly low. For example,
/// Redis running on an entry level laptop can scan a
/// 1 million key database in 40 milliseconds.
///
/// Warning: consider KEYS as a command that should
/// only be used in production environments with
/// extreme care. It may ruin performance when it
/// is executed against large databases. This command
/// is intended for debugging and special operations,
/// such as changing your keyspace layout.
/// # Arguments
/// * pattern - String
/// Supported glob-style patterns:
/// ``` h?llo matches hello, hallo and hxllo
/// h*llo matches hllo and heeeello
/// h[ae]llo matches hello and hallo, but not hillo
/// h[^e]llo matches hallo, hbllo, ... but not hello
/// h[a-b]llo matches hallo and hbllo
/// Use \ to escape special characters if you want to match them verbatim.
/// ```
/// # Return value
/// Array reply: list of keys matching pattern.
pub struct RedisCommandKeys {
    pattern: String,
}

impl RedisCommandKeys {
    pub fn new(key: String) -> RedisCommandKeys {
        RedisCommandKeys { pattern: key }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command Keys with pattern : {}",
            self.pattern
        ));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Keys(self.pattern.clone()))?;
        let response = match response.get_value() {
            StorageResult::Vector(vec) => vec.protocol_serialize_to_bulk_string(),
            _ => RedisError::Unknown.protocol_serialize_to_simple_string(),
        };
        verbose.print("Finalizing execution of command Keys");
        Ok(response)
    }
}
