use crate::global_resources::GlobalResources;

/// Subscribes the client to a given channel.
///
/// # Arguments
/// * key - String
///
/// # Return value
///

pub struct RedisCommandSubscribe {
    _channel: Vec<String>,
}

impl RedisCommandSubscribe {
    pub fn new(channel: Vec<String>) -> RedisCommandSubscribe {
        RedisCommandSubscribe { _channel: channel }
    }
    pub fn execute(&self, _global_resources: GlobalResources) -> Result<String, String> {
        todo!()
    }
}
