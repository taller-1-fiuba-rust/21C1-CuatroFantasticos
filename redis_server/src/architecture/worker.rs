use crate::architecture::message::Message;
use crate::configuration::verbose::Verbose;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        verbose: Verbose,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    verbose.print(&format!("Worker: {} got a job; Executing", id));
                    job();
                }
                Message::Terminate => {
                    verbose.print(&format!("Worker: {} is terminating", id));
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn thread(&mut self) -> &mut Option<JoinHandle<()>> {
        &mut self.thread
    }
}
