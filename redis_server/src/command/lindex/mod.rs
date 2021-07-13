use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;
use crate::data::storage::response_error_enum::ResponseErrorEnum;
use crate::data::storage::response_message::StorageResponseMessageEnum;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandLindex {
    key: String,
    index: String,
}

impl RedisCommandLindex {
    pub fn new(key: String, index: String) -> RedisCommandLindex {
        RedisCommandLindex { key, index }
    }
}

impl RedisCommand for RedisCommandLindex {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = match self.index.parse::<i32>() {
            Ok(index) => {
                let response =
                    accessor.access(StorageRequestMessageEnum::Lindex(self.key.clone(), index))?;
                match response.get_value() {
                    StorageResponseMessageEnum::String(value) => {
                        value.protocol_serialize_to_bulk_string()
                    }
                    value => value.protocol_serialize_to_bulk_string(),
                }
            }
            Err(_) => StorageResponseMessageEnum::Error(ResponseErrorEnum::NotANumber)
                .protocol_serialize_to_bulk_string(),
        };
        Ok(response)
    }
}
