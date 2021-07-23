use std::io::Write;
use std::sync::mpsc;

use crate::log_service::message::LogMessage;

pub struct LogWriter<T: Write> {
    log_receiver: mpsc::Receiver<LogMessage<T>>,
    output_buffer: T,
}

impl<T: Write> LogWriter<T> {
    pub fn new(log_receiver: mpsc::Receiver<LogMessage<T>>, output_buffer: T) -> Self {
        LogWriter {
            log_receiver,
            output_buffer,
        }
    }
    pub fn init(mut self) {
        for message in self.log_receiver {
            match message {
                LogMessage::Log(log) => {
                    if let Err(e) = writeln!(&mut self.output_buffer, "{}", log) {
                        eprintln!("Logging error: {}", e);
                    }
                }
                LogMessage::Terminate => {
                    break;
                }
                LogMessage::SetLogFile(file) => {
                    self.output_buffer = file;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;

    use crate::log_service::message::LogMessage;
    use crate::log_service::test_resources::VectorWriter;
    use crate::log_service::writer::LogWriter;

    #[test]
    fn log_writer_create() {
        let (_tx, rx) = mpsc::channel();
        let output_buffer = VectorWriter::new();
        let _log_writer = LogWriter::new(rx, output_buffer);
        assert!(true)
    }

    #[test]
    fn log_writer_init_and_shutdown() {
        let (tx, rx) = mpsc::channel();
        let output_buffer = VectorWriter::new();
        let log_writer_thread_handler = thread::spawn(move || {
            let log_writer = LogWriter::new(rx, output_buffer);
            log_writer.init();
        });
        tx.send(LogMessage::Terminate)
            .expect("Error sending termination message");
        log_writer_thread_handler
            .join()
            .expect("Error joining threads");
        assert!(true)
    }

    #[test]
    fn log_writer_receive_messages() {
        let (tx, rx) = mpsc::channel();
        let output_buffer = VectorWriter::new();
        let vector_writes = output_buffer.get_vector_copy();
        let log_writer_thread_handler = thread::spawn(move || {
            let log_writer = LogWriter::new(rx, output_buffer);
            log_writer.init();
        });
        tx.send(LogMessage::Log("first_log".to_string()))
            .expect("Error sending first log");
        tx.send(LogMessage::Log("second_log".to_string()))
            .expect("Error sending second log");
        tx.send(LogMessage::Log("third_log".to_string()))
            .expect("Error sending third log");
        tx.send(LogMessage::Log("fourth_log".to_string()))
            .expect("Error sending fourth log");
        tx.send(LogMessage::Terminate)
            .expect("Error sending termination message");
        log_writer_thread_handler
            .join()
            .expect("Error joining threads");
        let vector_writes = vector_writes
            .lock()
            .expect("Error getting vector of writes");
        assert_eq!("first_log", vector_writes[0]);
        assert_eq!("\n", vector_writes[1]);
        assert_eq!("second_log", vector_writes[2]);
        assert_eq!("\n", vector_writes[3]);
        assert_eq!("third_log", vector_writes[4]);
        assert_eq!("\n", vector_writes[5]);
        assert_eq!("fourth_log", vector_writes[6]);
        assert_eq!("\n", vector_writes[7]);
    }

    #[test]
    fn log_writer_terminates_when_no_senders_are_left() {
        let (tx, rx) = mpsc::channel();
        let output_buffer = VectorWriter::new();
        let log_writer_thread_handler = thread::spawn(move || {
            let log_writer = LogWriter::new(rx, output_buffer);
            log_writer.init();
        });
        drop(tx);
        log_writer_thread_handler
            .join()
            .expect("Error joining threads");
        assert!(true);
    }
}
