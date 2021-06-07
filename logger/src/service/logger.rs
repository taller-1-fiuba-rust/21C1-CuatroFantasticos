use std::error::Error;
use std::fs;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

enum LogMessage {
    Log(String),
    Terminate,
}

struct LogWriter<T: Write> {
    log_receiver: mpsc::Receiver<LogMessage>,
    output_buffer: T,
}
impl<T: Write> LogWriter<T> {
    fn new(log_receiver: mpsc::Receiver<LogMessage>, output_buffer: T) -> Self {
        LogWriter {
            log_receiver,
            output_buffer,
        }
    }
    fn init(mut self) {
        for message in self.log_receiver {
            match message {
                LogMessage::Log(log) => {
                    if let Err(e) = write!(&mut self.output_buffer, "{}", log) {
                        eprintln!("Logging error: {}", e);
                    }
                }
                LogMessage::Terminate => {
                    break;
                }
            }
        }
    }
}

pub struct Logger {
    log_sender: mpsc::Sender<LogMessage>,
}
pub enum LogError {
    LogError,
}
impl Logger {
    fn new(log_sender: mpsc::Sender<LogMessage>) -> Self {
        Logger { log_sender }
    }
    pub fn log(self, log_string: String) -> Result<(), LogError> {
        match self.log_sender.send(LogMessage::Log(log_string)) {
            Ok(_) => Ok(()),
            Err(_) => Err(LogError::LogError),
        }
    }
}
pub struct LogService {
    log_sender: mpsc::Sender<LogMessage>,
    log_writer_thread_handler: Option<JoinHandle<()>>,
}
impl LogService {
    pub fn new(output_file_path: &str) -> Result<Self, Box<dyn Error>> {
        let output_buffer = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_file_path)?;
        let (log_sender, log_receiver) = mpsc::channel();
        let log_writer_thread_handler = thread::spawn(move || {
            let log_writer = LogWriter::new(log_receiver, output_buffer);
            log_writer.init();
        });
        Ok(LogService {
            log_sender,
            log_writer_thread_handler: Some(log_writer_thread_handler),
        })
    }

    pub fn create_logger(&self) -> Logger {
        Logger::new(self.log_sender.clone())
    }
}

impl Drop for LogService {
    fn drop(&mut self) {
        if self.log_sender.send(LogMessage::Terminate).is_err() {
            return;
        }
        if let Some(log_writer_thread_handler) = self.log_writer_thread_handler.take() {
            log_writer_thread_handler.join().unwrap();
        }
    }
}
