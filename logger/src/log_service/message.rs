use std::io::Write;

#[derive(Clone)]
pub enum LogMessage<T: Write> {
    Log(String),
    SetLogFile(T),
    Terminate,
}
