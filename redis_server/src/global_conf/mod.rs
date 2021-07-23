use std::sync::mpsc;
use std::thread::JoinHandle;

use logger::log_service::log_interface::LogInterface;
use std::fs::File;
use crate::configuration::conf_request_message::ConfMessage;

pub struct GlobalConf {
    logger_builder : LogInterface<File>,
    configuration_sender: mpsc::Sender<ConfMessage>,
    configuration_thread_handler : Option<JoinHandle<()>>,
}