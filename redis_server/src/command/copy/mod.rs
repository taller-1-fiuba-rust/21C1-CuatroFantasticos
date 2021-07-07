use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;
use crate::protocol_serialization::ProtocolSerializer;

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
        let copy = accessor.access(StorageRequestMessageEnum::Copy(
            self.source_key.clone(),
            self.destination_key.clone(),
        ))?;
        let response = copy.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}