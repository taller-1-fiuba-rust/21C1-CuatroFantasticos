use crate::configuration::service::request_message::{ConfAction, ConfRequestMessage};
use crate::configuration::service::response_message::{ConfError, ConfResult};
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

    fn validate_key(&self, key: &str) -> Result<String, ()> {
        match key {
            "verbose" | "dbfilename" | "timeout" | "logfile" => Ok(key.to_owned()),
            _ => Err(()),
        }
    }

    pub fn init(mut self) {
        for message in &self.conf_receiver {
            match message.get_message() {
                ConfAction::Get => {
                    let _ = message.respond(ConfResult::OkConf(self.configuration.clone()));
                }
                ConfAction::Set(key, value) => match self.validate_key(&key) {
                    Ok(key) => {
                        self.configuration.set(key, value);
                        let _ = message.respond(ConfResult::Ok);
                    }
                    Err(_) => {
                        let _ = message.respond(ConfResult::Error(ConfError::NonExistent));
                    }
                },
                ConfAction::GetParameter(value) => match self.configuration.get(&value) {
                    Some(value) => {
                        let _ = message.respond(ConfResult::OkParameter(value.clone()));
                    }
                    None => {
                        let _ = message.respond(ConfResult::Error(ConfError::NonExistent));
                    }
                },
                ConfAction::Terminate => {
                    break;
                }
            }
        }
    }
}
