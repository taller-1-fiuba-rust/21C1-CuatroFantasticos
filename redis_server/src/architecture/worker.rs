use crate::architecture::message::Message;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

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
#[cfg(test)]
mod test {
    use crate::architecture::worker::Worker;


#[test]
    fn create_new_worker_has_no_errors() {

    //let a = Worker::new(2,);
}

}