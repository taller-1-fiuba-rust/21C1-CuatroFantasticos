use std::sync::mpsc;
use crate::configuration::Configuration;
use crate::configuration::conf_request_message::{ConfMessage, ConfRequestMessage};
use crate::configuration::conf_response_message::{ConfResult, ConfError};

pub struct ConfWorker {
    conf_receiver: mpsc::Receiver<ConfRequestMessage>,
    configuration : Configuration,
}

impl ConfWorker{

    pub fn new(conf_receiver: mpsc::Receiver<ConfRequestMessage>, configuration: Configuration) -> Self {
        ConfWorker {
            conf_receiver,
            configuration,
        }
    }

    pub fn validate_key(&self, key: &str) -> Result<String,()>{
        match key {
            "verbose"| "dbfilename"| "timeout" | "logfile" => Ok(key.to_owned()),
            _ => Err(()),
        }
    }


    pub fn init(mut self) {
        for message in &self.conf_receiver {
            match message.get_message() {
                ConfMessage::Get(value) => {
                    match self.configuration.get(&value){
                        Some(value) => {
                            let response = ConfResult::OkString(value);
                            let _ = message.respond(response);
                        }
                        None => {
                            let response = ConfResult::Error(ConfError::NonExistent);
                            let _ = message.respond(response);
                        }
                    }
                }
                ConfMessage::Set(key,value) => {
                    match self.validate_key(&key){
                        Ok(key) => {
                            self.configuration.set(key,value);
                            let _ = message.respond(ConfResult::Ok);
                        }
                        Err(_) => {
                            let _ = message.respond(ConfResult::Error(ConfError::NonExistent));
                        }
                    }
                }
                ConfMessage::Terminate => {
                    break;
                }
            }
        }
    }

}