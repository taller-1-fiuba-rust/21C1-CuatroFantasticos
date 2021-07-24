use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::data::storage_service::operator_service::response_message::StorageResult;
use crate::data::storage_service::operator_service::result_error::RedisError;
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
}

impl RedisCommand for RedisCommandKeys {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Keys(self.pattern.clone()))?;
        let response = match response.get_value() {
            StorageResult::Vector(vec) => vec.protocol_serialize_to_simple_string(),
            _ => RedisError::Unknown.protocol_serialize_to_simple_string(),
        };
        Ok(response)
    }
}
