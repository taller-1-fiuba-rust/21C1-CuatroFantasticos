use crate::configuration::request_message::{ConfMessage, ConfRequestMessage};
use crate::configuration::response_message::{ConfError, ConfResult};
use crate::configuration::Configuration;
use std::sync::mpsc;

pub struct ConfWorker {
    conf_receiver: mpsc::Receiver<ConfRequestMessage>,
    configuration: Configuration,
}

impl ConfWorker {
    pub fn new(
        conf_receiver: mpsc::Receiver<ConfRequestMessage>,
        configuration: Configuration,
    ) -> Self {
        ConfWorker {
            conf_receiver,
            configuration,
        }
    }

    pub fn validate_key(&self, key: &str) -> Result<String, ()> {
        match key {
            "verbose" | "dbfilename" | "timeout" | "logfile" => Ok(key.to_owned()),
            _ => Err(()),
        }
    }

    pub fn init(mut self) {
        for message in &self.conf_receiver {
            match message.get_message() {
                ConfMessage::Get => {
                    let _ = message.respond(ConfResult::OkConf(self.configuration.clone()));
                }
                ConfMessage::Set(key, value) => match self.validate_key(&key) {
                    Ok(key) => {
                        self.configuration.set(key, value);
                        let _ = message.respond(ConfResult::Ok);
                    }
                    Err(_) => {
                        let _ = message.respond(ConfResult::Error(ConfError::NonExistent));
                    }
                },
                ConfMessage::Terminate => {
                    break;
                }
            }
        }
    }
}
