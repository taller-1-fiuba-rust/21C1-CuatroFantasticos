#[derive(Clone)]
pub enum LogMessage {
    Log(String),
    Terminate,
}
