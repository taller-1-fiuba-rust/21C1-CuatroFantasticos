use crate::log_service::logger::Logger;
use crate::log_service::message::LogMessage;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::mpsc;

#[derive(Debug)]
pub struct LogInterface<T: Write> {
    log_sender: mpsc::Sender<LogMessage<T>>,
}

pub struct LogInterfaceError {}

impl<T: Write> LogInterface<T> {
    pub fn new(log_sender: mpsc::Sender<LogMessage<T>>) -> Self {
        LogInterface { log_sender }
    }
    pub fn build_logger(&self) -> Logger<T> {
        Logger::new(self.log_sender.clone())
    }
    pub fn set_log_output_object(&self, output_buffer: T) -> Result<(), LogInterfaceError> {
        self.log_sender
            .send(LogMessage::SetLogFile(output_buffer))
            .map_err(|_| LogInterfaceError {})
    }
}

impl LogInterface<File> {
    pub fn set_log_file(&self, path: &str) -> Result<(), LogInterfaceError> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .map_err(|_| LogInterfaceError {})?;
        self.log_sender
            .send(LogMessage::SetLogFile(file))
            .map_err(|_| LogInterfaceError {})
    }
}

impl<T: Write> Clone for LogInterface<T> {
    fn clone(&self) -> Self {
        LogInterface::new(self.log_sender.clone())
    }
}
