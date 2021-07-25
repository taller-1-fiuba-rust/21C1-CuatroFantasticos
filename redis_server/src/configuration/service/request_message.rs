use crate::configuration::service::response_message::ConfResult;
use std::sync::mpsc;

pub struct ConfRequestMessage {
    message: ConfAction,
    sender: Option<mpsc::Sender<ConfResult>>,
}

impl ConfRequestMessage {
    pub fn new(
        message: ConfAction,
        sender: Option<mpsc::Sender<ConfResult>>,
    ) -> ConfRequestMessage {
        ConfRequestMessage { message, sender }
    }

    pub fn get_message(&self) -> ConfAction {
        self.message.clone()
    }

    pub fn respond(&self, response: ConfResult) -> Result<(), String> {
        match &self.sender {
            Some(sender) => sender
                .send(response)
                .map_err(|_| "Conf accessor is not listening to conf response".to_string()),
            None => Err("There is no sender present to respond".to_string()),
        }
    }
}
#[derive(Clone)]
pub enum ConfAction {
    Terminate,
    Get,
    GetParameter(String),
    Set(String, String),
}
