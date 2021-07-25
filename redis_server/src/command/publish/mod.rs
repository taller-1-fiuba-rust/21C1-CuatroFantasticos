use crate::global_resources::GlobalResources;

/// Publish the message to a given channel.
///
/// # Arguments
/// * message - String
/// * channel - String
///
/// # Return value
///

pub struct RedisCommandPublish {
    _message: String,
    _channel: String,
}

impl RedisCommandPublish {
    pub fn new(message: String, channel: String) -> RedisCommandPublish {
        RedisCommandPublish {
            _message: message,
            _channel: channel,
        }
    }
    pub fn execute(&self, _global_resources: GlobalResources) -> Result<String, String> {
        todo!()
    }
}
