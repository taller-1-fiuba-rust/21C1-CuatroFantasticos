use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;
///By default, FLUSHDB will synchronously flush all keys from the database.
/// Starting with Redis 6.2, setting the lazyfree-lazy-user-flush configuration directive to
/// "yes" changes the default flush mode to asynchronous.
///It is possible to use one of the following modifiers to dictate the flushing mode explicitly:
///ASYNC: flushes the database asynchronously
///SYNC: flushes the database synchronously
///
/// # Return value
/// Simple string reply
pub struct RedisCommandFlushDb {}

impl RedisCommandFlushDb {
    pub fn new() -> RedisCommandFlushDb {
        RedisCommandFlushDb {}
    }
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::FlushDb)?;
        let response = response.get_value().protocol_serialize_to_simple_string();
        Ok(response)
    }
}

impl Default for RedisCommandFlushDb {
    fn default() -> Self {
        Self::new()
    }
}
