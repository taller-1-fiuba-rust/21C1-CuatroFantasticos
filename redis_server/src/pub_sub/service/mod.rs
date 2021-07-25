mod action;
mod request_message;
mod result;

use crate::pub_sub::orchestrator::PubSubOrchestrator;
use crate::pub_sub::service::action::PubSubAction;
use crate::pub_sub::service::request_message::PubSubOperatorRequest;
use crate::pub_sub::service::result::PubSubResult;
use std::sync::mpsc;

pub struct PubSubOperator {
    orchestrator: PubSubOrchestrator,
    receiver: mpsc::Receiver<PubSubOperatorRequest>,
}

impl PubSubOperator {
    pub fn new(receiver: mpsc::Receiver<PubSubOperatorRequest>) -> Self {
        let orchestrator = PubSubOrchestrator::new();
        PubSubOperator {
            orchestrator,
            receiver,
        }
    }
    pub fn init(mut self) {
        for request in self.receiver {
            match request.get_action() {
                PubSubAction::Subscribe(subscriptor, channel) => {
                    self.orchestrator
                        .subscribe_to_channel(subscriptor, &channel);
                    let _ = request.respond(PubSubResult::Ok);
                }
                PubSubAction::Unsubscribe(subscriptor, channel) => {
                    self.orchestrator
                        .unsubscribe_from_channel(subscriptor, &channel);
                    let _ = request.respond(PubSubResult::Ok);
                }
                PubSubAction::UnsubscribeAll(_subscriptor) => {
                    todo!()
                }
                PubSubAction::Publish(message, channel) => {
                    let receiver_qty = self.orchestrator.publish_to_channel(&message, &channel);
                    let _ = request.respond(PubSubResult::IntegerReply(receiver_qty));
                }
            }
        }
    }
}
