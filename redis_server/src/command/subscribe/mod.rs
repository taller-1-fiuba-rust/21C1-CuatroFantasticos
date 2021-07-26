use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;
use crate::pub_sub::service::action::PubSubAction;
use crate::pub_sub::service::result::PubSubResult;
use crate::pub_sub::subscriptor::PubSubSubscriptor;

/// Subscribes the client to a given channel.
///
/// # Arguments
/// * key - String
///
/// # Return value
///

pub struct RedisCommandSubscribe {
    channels: Vec<String>,
}

impl RedisCommandSubscribe {
    pub fn new(channel: Vec<String>) -> RedisCommandSubscribe {
        RedisCommandSubscribe { channels: channel }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources
            .get_verbose()
            .expect("Could not get verbose");
        let client_accessor = global_resources
            .get_client_accessor()
            .ok_or_else(|| "-Unexpected error executing SUBSCRIBE".to_owned())?;

        verbose.print(&format!(
            "Executing command Subscribe for client {{{:?}}} and channels {{{:?}}} ",
            client_accessor.get_client_id(),
            self.channels
        ));

        let subscriptor = PubSubSubscriptor::new(client_accessor.get_client_id(), client_accessor);

        let mut response = "".to_owned();
        for channel in &self.channels {
            let pub_sub_response =
                global_resources
                    .get_pub_sub_accessor()
                    .access(PubSubAction::Subscribe(
                        subscriptor.clone(),
                        channel.clone(),
                    ))?;
            let protocolized_partial_response = match pub_sub_response {
                PubSubResult::SubscriptionResult(r) => r.protocolize(),
                _ => {
                    return Err(RedisError::Unknown.protocol_serialize_to_bulk_string());
                }
            };
            response += &protocolized_partial_response;
        }
        verbose.print("Finalizing execution of command subscribe");
        Ok(response)
    }
}
