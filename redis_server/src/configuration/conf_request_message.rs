use std::sync::mpsc;
use crate::configuration::conf_response_message::ConfResult;

pub struct ConfRequestMessage {
    message: ConfMessage,
    sender: Option<mpsc::Sender<ConfResult>>,
}

impl ConfRequestMessage {
    pub fn new(
        message: ConfMessage,
        sender: Option<mpsc::Sender<ConfResult>>,
    ) -> ConfRequestMessage {
        ConfRequestMessage { message, sender }
    }

    pub fn get_message(&self) -> ConfMessage {
        self.message.clone()
    }

    pub fn respond(&self, response: ConfResult) -> Result<(), String> {
        match &self.sender {
            Some(sender) => {
                sender
                    .send(response)
                    .map_err(|_| "Conf accessor is not listening to conf response".to_string())
            }
            None => Err("There is no sender present to respond".to_string()),
        }
    }
}
#[derive(Clone)]
pub enum ConfMessage {
    Terminate,
    Get(String),
    Set(String,String),
}

