use std::error::Error;
use std::fs;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::SendError;
use std::thread;

pub struct LogWriter<T: Write> {
    log_receiver: mpsc::Receiver<String>,
    output_buffer: T,
}
impl<T: Write> LogWriter<T> {
    pub fn new(log_receiver: mpsc::Receiver<String>, output_buffer: T) -> Self {
        LogWriter {
            log_receiver,
            output_buffer,
        }
    }
    pub fn init(mut self) {
        for message in self.log_receiver {
            if let Err(e) = write!(&mut self.output_buffer, "{}", message) {
                eprintln!("Logging error: {}", e);
            }
        }
    }
}

pub struct Logger {
    log_sender: mpsc::Sender<String>,
}
impl Logger {
    pub fn new(log_sender: mpsc::Sender<String>) -> Self {
        Logger { log_sender }
    }
    pub fn log(self, log_string: String) -> Result<(), SendError<String>> {
        self.log_sender.send(log_string)
    }
}
pub struct LogService {
    log_sender: mpsc::Sender<String>,
}
pub enum LogServiceError {}
impl LogService {
    pub fn new(output_file_path: &str) -> Result<Self, Box<dyn Error>> {
        let output_buffer = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_file_path)?;
        let (log_sender, log_receiver) = mpsc::channel();
        let _ = thread::spawn(move || {
            let log_writer = LogWriter::new(log_receiver, output_buffer);
            log_writer.init();
        });
        Ok(LogService { log_sender })
    }

    pub fn create_logger(&self) -> Logger {
        Logger::new(self.log_sender.clone())
    }
}
