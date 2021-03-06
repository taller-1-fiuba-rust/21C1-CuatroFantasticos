use crate::log_service::log_interface::LogInterface;
use message::LogMessage;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use writer::LogWriter;

pub mod log_interface;
pub mod logger;

pub mod message;
mod writer;

#[cfg(test)]
mod test_resources;

#[derive(Debug)]
pub struct LogService<T: Write + Send + 'static> {
    log_sender: mpsc::Sender<LogMessage<T>>,
    log_writer_thread_handler: Option<JoinHandle<()>>,
}

impl<T: Write + Send + 'static> LogService<T> {
    pub fn new(output_buffer: T) -> Self {
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

    pub fn get_log_interface(&self) -> LogInterface<T> {
        LogInterface::new(self.log_sender.clone())
    }
}

impl LogService<File> {
    pub fn new_with_path(path: &str) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .expect("Log service: Could not open log file");
        let (log_sender, log_receiver) = mpsc::channel();
        let log_writer_thread_handler = thread::spawn(move || {
            let log_writer = LogWriter::new(log_receiver, file);
            log_writer.init();
        });
        LogService {
            log_sender,
            log_writer_thread_handler: Some(log_writer_thread_handler),
        }
    }
}

impl<T: Write + Send + 'static> Drop for LogService<T> {
    fn drop(&mut self) {
        let _ = self.log_sender.send(LogMessage::Terminate);
        if let Some(log_writer_thread_handler) = self.log_writer_thread_handler.take() {
            log_writer_thread_handler.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::log_service::test_resources::VectorWriter;
    use crate::log_service::LogService;

    #[test]
    fn log_service_create() {
        let log_output = VectorWriter::new();
        let _ = LogService::new(log_output);
    }

    #[test]
    fn log_logger_create() {
        let log_output = VectorWriter::new();
        let log_service = LogService::new(log_output);
        let _ = log_service.get_log_interface().build_logger();
    }

    #[test]
    fn log_logger_log() {
        let log_output = VectorWriter::new();
        let vector_logs = log_output.get_vector_copy();
        {
            let log_service = LogService::new(log_output);
            let logger = log_service.get_log_interface().build_logger();
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
