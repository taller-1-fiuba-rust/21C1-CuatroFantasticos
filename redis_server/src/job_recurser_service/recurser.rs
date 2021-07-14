use crate::job_recurser_service::RecurringJob;
use crate::utilities::current_time_in_millis;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

const WAIT_TIME: u64 = 100;

pub struct JobRecurser<T: RecurringJob + Send> {
    job: T,
    receiver: mpsc::Receiver<JobRecurserMessage>,
    period: u128,
    next_ts: u128,
}

pub enum JobRecurserMessage {
    Terminate,
}

impl<T: RecurringJob + Send> JobRecurser<T> {
    pub fn new(
        job: T,
        receiver: mpsc::Receiver<JobRecurserMessage>,
        period: u128,
    ) -> JobRecurser<T> {
        let next_ts = current_time_in_millis() + period;
        JobRecurser {
            job,
            receiver,
            period,
            next_ts,
        }
    }

    fn update_next_ts(&mut self) {
        self.next_ts += self.period;
    }

    fn wait(&self) {
        let current_time = current_time_in_millis();
        if current_time + (WAIT_TIME as u128) < self.next_ts {
            sleep(Duration::from_millis(WAIT_TIME));
        } else {
            sleep(Duration::from_millis((self.next_ts - current_time) as u64));
        }
    }

    pub fn init(&mut self) {
        loop {
            let message = self.receiver.try_recv();
            if let Ok(JobRecurserMessage::Terminate) = message {
                break;
            }
            if self.next_ts <= current_time_in_millis() && self.job.execute_job().is_err() {
                break;
            }
            self.update_next_ts();
            self.wait();
        }
    }
}
