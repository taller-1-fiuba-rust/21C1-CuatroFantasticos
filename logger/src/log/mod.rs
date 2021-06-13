use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

use crate::log::logger::Logger;
use message::LogMessage;
use writer::LogWriter;

pub mod logger;
pub mod message;
#[cfg(test)]
mod test_resources;
mod writer;

#[derive(Debug)]
pub struct LogService {
    log_sender: mpsc::Sender<LogMessage>,
    log_writer_thread_handler: Option<JoinHandle<()>>,
}

impl LogService {
    pub fn new<T: Write + Send + 'static>(output_buffer: T) -> Self {
        let (log_sender, log_receiver) = mpsc::channel();
        let log_writer_thread_handler = thread::spawn(move || {
            let log_writer = LogWriter::new(log_receiver, output_buffer);
            log_writer.init();
        });
        LogService {
            log_sender,
            log_writer_thread_handler: Some(log_writer_thread_handler),
        }
    }

    pub fn create_logger(&self) -> Logger {
        Logger::new(self.log_sender.clone())
    }
}

impl Drop for LogService {
    fn drop(&mut self) {
        let _ = self.log_sender.send(LogMessage::Terminate);
        if let Some(log_writer_thread_handler) = self.log_writer_thread_handler.take() {
            log_writer_thread_handler.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::log::test_resources::VectorWriter;
    use crate::log::LogService;

    #[test]
    fn log_service_create() {
        let log_output = VectorWriter::new();
        let _ = LogService::new(log_output);
    }

    #[test]
    fn log_logger_create() {
        let log_output = VectorWriter::new();
        let log_service = LogService::new(log_output);
        let _ = log_service.create_logger();
    }

    #[test]
    fn log_logger_log() {
        let log_output = VectorWriter::new();
        let vector_logs = log_output.get_vector_copy();
        {
            let log_service = LogService::new(log_output);
            let logger = log_service.create_logger();
            logger.log("first_log").expect("Error in first log");
            logger.log("second_log").expect("Error in second log");
            logger.log("third_log").expect("Error in third log");
            logger.log("fourth_log").expect("Error in fourth log");
        }
        let vector_logs = vector_logs.lock().expect("Error getting vector of writes");
        assert_eq!("first_log", vector_logs[0]);
        assert_eq!("\n", vector_logs[1]);
        assert_eq!("second_log", vector_logs[2]);
        assert_eq!("\n", vector_logs[3]);
        assert_eq!("third_log", vector_logs[4]);
        assert_eq!("\n", vector_logs[5]);
        assert_eq!("fourth_log", vector_logs[6]);
        assert_eq!("\n", vector_logs[7]);
    }
}
