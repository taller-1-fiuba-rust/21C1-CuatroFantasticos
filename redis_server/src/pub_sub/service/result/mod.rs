use crate::pub_sub::service::result::subscription::PubSubSubscriptionResult;

pub mod subscription;

pub enum PubSubResult {
    IntegerReply(usize),
    SubscriptionResult(PubSubSubscriptionResult),
    Ok,
}
