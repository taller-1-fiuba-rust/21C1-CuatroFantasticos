use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;
use crate::pub_sub::service::action::PubSubAction;
use crate::pub_sub::service::result::PubSubResult;

/// Publish the message to a given channel.
///
/// # Arguments
/// * message - String
/// * channel - String
///
/// # Return value
///

pub struct RedisCommandPublish {
    channel: String,
    message: String,
}

impl RedisCommandPublish {
    pub fn new(channel: String, message: String) -> RedisCommandPublish {
        RedisCommandPublish { channel, message }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        let client_accessor = global_resources
            .get_client_accessor()
            .ok_or_else(|| "-Unexpected error executing PUBLISH".to_owned())?;

        verbose.print(&format!(
            "Executing command Publish with message {{{:?}}} and channel {{{:?}}} ",
            client_accessor.get_client_id(),
            self.channel
        ));

        let response = global_resources
            .get_pub_sub_accessor()
            .access(PubSubAction::Publish(
                self.channel.clone(),
                self.message.clone(),
            ))?;

        let response = match response {
            PubSubResult::IntegerReply(value) => value.to_string().protocol_serialize_to_int(),
            _ => {
                return Err(RedisError::Unknown.protocol_serialize_to_bulk_string());
            }
        };

        verbose.print("Finalizing execution of command publish");
        Ok(response)
    }
}
