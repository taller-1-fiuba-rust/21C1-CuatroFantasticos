use crate::pub_sub::subscriptor::PubSubSubscriptor;

#[derive(Clone)]
pub enum PubSubAction {
    Subscribe(PubSubSubscriptor, String),
    Unsubscribe(PubSubSubscriptor, String),
    UnsubscribeAll(PubSubSubscriptor),
    Publish(String, String),
}
