use crate::log::message::LogMessage;
use core::result::Result;
use core::result::Result::{Err, Ok};
use std::error::Error;
use std::sync::mpsc;

#[derive(Clone, Debug)]
pub struct Logger {
    log_sender: mpsc::Sender<LogMessage>,
}

impl Logger {
    pub fn new(log_sender: mpsc::Sender<LogMessage>) -> Self {
        Logger { log_sender }
    }
    pub fn log(&self, log_string: &str) -> Result<(), Box<dyn Error>> {
        match self
            .log_sender
            .send(LogMessage::Log(log_string.to_string()))
        {
            Ok(_) => Ok(()),
            Err(_) => Err("Error logging".into()),
        }
    }
}
