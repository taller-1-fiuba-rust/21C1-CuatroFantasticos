use crate::configuration::Configuration;
use crate::configuration::verbose::Verbose;

pub struct ConfResponseMessage {
    value_response: ConfResult,
}

impl ConfResponseMessage {
    pub fn new(value_response: ConfResult) -> ConfResponseMessage {
        ConfResponseMessage { value_response }
    }

    pub fn get_value(&self) -> &ConfResult {
        &self.value_response
    }
}

pub enum ConfResult {
    Ok,
    Vector(Vec<String>),
    Verbose(Verbose),
    OkConf(Configuration),
    Error(ConfError),
}

pub enum ConfError {
    NonExistent,
}
