use crate::configuration::accesor::ConfAccessor;
use crate::configuration::accessor_builder::ConfAccessorBuilder;
use crate::configuration::request_message::{ConfMessage, ConfRequestMessage};
use crate::configuration::response_message::ConfResult;
use crate::configuration::worker::ConfWorker;
use crate::configuration::Configuration;
use std::sync::mpsc;
use std::thread;

pub struct ConfService {
    conf_request_sender: mpsc::Sender<ConfRequestMessage>,
    conf_thread_handler: Option<thread::JoinHandle<()>>,
}

impl ConfService {
    pub fn new(conf_filename: String) -> Self {
        let configuration = Configuration::new(&conf_filename);
        let (conf_tx, conf_rx) = mpsc::channel::<ConfRequestMessage>();

        let conf_th = thread::spawn(move || {
            let conf = ConfWorker::new(conf_rx, configuration);
            conf.init();
        });

        ConfService {
            conf_request_sender: conf_tx,
            conf_thread_handler: Some(conf_th),
        }
    }

    pub fn get_accessor_builder(&self) -> ConfAccessorBuilder {
        ConfAccessorBuilder::new(self.conf_request_sender.clone())
    }

    pub fn get_conf(&self) -> Result<Configuration, String> {
        let accessor = ConfAccessor::new(self.conf_request_sender.clone());
        match accessor.access(ConfMessage::Get).unwrap() {
            ConfResult::OkConf(value) => Ok(value),
            _ => Err(String::from("Couldn't get a configuration")),
        }
    }
}

impl Drop for ConfService {
    fn drop(&mut self) {
        let _ = self
            .conf_request_sender
            .send(ConfRequestMessage::new(ConfMessage::Terminate, None));
        if let Some(th) = self.conf_thread_handler.take() {
            th.join().unwrap();
        }
    }
}
