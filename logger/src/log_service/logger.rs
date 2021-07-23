use crate::log_service::message::LogMessage;
use core::result::Result;
use core::result::Result::{Err, Ok};
use std::error::Error;
use std::io::Write;
use std::sync::mpsc;

#[derive(Clone, Debug)]
pub struct Logger<T: Write> {
    log_sender: mpsc::Sender<LogMessage<T>>,
}

impl<T: Write> Logger<T> {
    pub fn new(log_sender: mpsc::Sender<LogMessage<T>>) -> Self {
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

// Testear

#[cfg(test)]

mod tests {

    use crate::log_service::logger::Logger;
    use crate::log_service::message::LogMessage;
    use std::fs::File;
    use std::sync::mpsc;

    #[test]
    fn new_logger_created() {
        let (sender, _receiver) = mpsc::channel::<LogMessage<File>>();
        Logger::new(sender);
    }
}
