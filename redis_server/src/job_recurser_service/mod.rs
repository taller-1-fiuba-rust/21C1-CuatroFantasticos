use crate::job_recurser_service::recurser::{JobRecurser, JobRecurserMessage};
use std::sync::mpsc;
use std::thread;

mod recurser;

pub struct RecurringJobError {}
pub trait RecurringJob {
    fn execute_job(&mut self) -> Result<(), RecurringJobError>;
}

pub struct JobRecurserService {
    request_sender: mpsc::Sender<JobRecurserMessage>,
    thread_handler: Option<thread::JoinHandle<()>>,
}

impl JobRecurserService {
    pub fn new<T: 'static + RecurringJob + Send>(job: T, period_ms: u128) -> JobRecurserService {
        let (tx, rx) = mpsc::channel::<JobRecurserMessage>();
        let th = thread::spawn(move || {
            let mut job_recurser = JobRecurser::new(job, rx, period_ms);
            job_recurser.init();
        });
        JobRecurserService {
            request_sender: tx,
            thread_handler: Some(th),
        }
    }
}

impl Drop for JobRecurserService {
    fn drop(&mut self) {
        let _ = self.request_sender.send(JobRecurserMessage::Terminate);
        if let Some(th) = self.thread_handler.take() {
            th.join().unwrap();
        }
    }
}
