use crate::command::RedisCommand;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;
///This command copies the value stored at the source key to the destination key
/// By default, the destination key is created in the logical database used by the connection.
/// The DB option allows specifying an alternative logical database index for the destination key.
/// The command returns an error when the destination key already exists.
/// The REPLACE option removes the destination key before copying the value to it.
///
/// # Arguments
///  * source_key - String,
///  * destination_key - String,
///
/// # Return value
///Integer reply, specifically:
/// * 1 if source was copied.
/// * 0 if source was not copied.

pub struct RedisCommandCopy {
    source_key: String,
    destination_key: String,
}

impl RedisCommandCopy {
    pub fn new(source_key: String, destination_key: String) -> RedisCommandCopy {
        RedisCommandCopy {
            source_key,
            destination_key,
        }
    }
}

impl RedisCommand for RedisCommandCopy {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let copy = accessor.access(StorageAction::Copy(
            self.source_key.clone(),
            self.destination_key.clone(),
        ))?;
        let response = copy.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
