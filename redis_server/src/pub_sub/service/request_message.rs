use crate::pub_sub::service::action::PubSubAction;
use crate::pub_sub::service::result::PubSubResult;
use std::fmt::{Debug, Formatter};
use std::sync::mpsc;

pub struct PubSubOperatorRequest {
    action: PubSubAction,
    response_sender: Option<mpsc::Sender<PubSubResult>>,
}

pub enum PubSubOperatorMessageError {
    TryRespondTerminationMessage,
    ResponseSendError,
}

impl Debug for PubSubOperatorMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PubSubOperatorMessageError::TryRespondTerminationMessage => {
                write!(f, "Pub Sub Operator: Can not respond a termination message")
            }
            PubSubOperatorMessageError::ResponseSendError => {
                write!(f, "Pub Sub Operator: Could not send response")
            }
        }
    }
}

impl From<PubSubOperatorMessageError> for String {
    fn from(e: PubSubOperatorMessageError) -> Self {
        format!("{:?}", e)
    }
}

impl PubSubOperatorRequest {
    pub fn new(action: PubSubAction, response_sender: Option<mpsc::Sender<PubSubResult>>) -> Self {
        PubSubOperatorRequest {
            action,
            response_sender,
        }
    }
    pub fn get_action(&self) -> PubSubAction {
        self.action.clone()
    }
    pub fn respond(&self, result: PubSubResult) -> Result<(), PubSubOperatorMessageError> {
        match &self.response_sender {
            Some(sender) => sender
                .send(result)
                .map_err(|_| PubSubOperatorMessageError::ResponseSendError),
            None => Err(PubSubOperatorMessageError::TryRespondTerminationMessage),
        }
    }
}
