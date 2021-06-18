use crate::architecture::message::Message;
use crate::architecture::worker::Worker;
use crate::configuration::Configuration;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool<'a> {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    conf: &'a Configuration,
}

impl<'a> ThreadPool<'a> {
    pub fn new(size: usize, conf: &Configuration) -> ThreadPool {
        conf.verbose("Threadpool: Creating Threadpool");
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), conf));
        }

        ThreadPool {
            workers,
            sender,
            conf,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl<'a> Drop for ThreadPool<'a> {
    fn drop(&mut self) {
        self.conf
            .verbose("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        self.conf.verbose("Shutting down all workers.");

        for worker in &mut self.workers {
            self.conf
                .verbose(&format!("Shutting down worker {}", worker.id()));

            if let Some(thread) = worker.thread().take() {
                thread.join().unwrap();
            }
        }
    }
}
