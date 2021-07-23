use std::sync::mpsc;
use crate::configuration::conf_request_message::{ConfRequestMessage, ConfMessage};
use std::thread;
use crate::configuration::Configuration;
use crate::configuration::conf_worker::ConfWorker;

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

    pub fn get_conf_sender(&self) -> mpsc::Sender<ConfRequestMessage> {
        self.conf_request_sender.clone()
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
