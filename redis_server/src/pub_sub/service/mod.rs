use crate::pub_sub::service::accessor_builder::PubSubAccessorBuilder;
use crate::pub_sub::service::action::PubSubAction;
use crate::pub_sub::service::operator::PubSubOperator;
use crate::pub_sub::service::request_message::PubSubOperatorRequest;
use std::sync::mpsc;
use std::thread;

pub mod accessor;
pub mod accessor_builder;
mod action;
mod operator;
mod request_message;
mod result;

pub struct PubSubService {
    operator_request_sender: mpsc::Sender<PubSubOperatorRequest>,
    operator_thread_handler: Option<thread::JoinHandle<()>>,
}

impl PubSubService {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_accessor_builder(&self) -> PubSubAccessorBuilder {
        PubSubAccessorBuilder::new(self.operator_request_sender.clone())
    }
}
impl Default for PubSubService {
    fn default() -> Self {
        let (operator_tx, operator_rx) = mpsc::channel::<PubSubOperatorRequest>();

        let operator_th = thread::spawn(move || {
            let operator = PubSubOperator::new(operator_rx);
            operator.init();
        });
        PubSubService {
            operator_request_sender: operator_tx,
            operator_thread_handler: Some(operator_th),
        }
    }
}
impl Drop for PubSubService {
    fn drop(&mut self) {
        let _ = self
            .operator_request_sender
            .send(PubSubOperatorRequest::new(PubSubAction::Terminate, None));
        if let Some(th) = self.operator_thread_handler.take() {
            th.join().unwrap();
        }
    }
}
