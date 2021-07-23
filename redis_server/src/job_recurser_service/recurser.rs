use crate::job_recurser_service::RecurringJob;
use crate::utilities::current_time_in_millis;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

const WAIT_TIME: u64 = 100;

#[derive(Debug)]
pub struct JobRecurser<T: RecurringJob + Send> {
    job: T,
    receiver: mpsc::Receiver<JobRecurserMessage>,
    period: u128,
    next_ts: u128,
    wait_time: u64,
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
            wait_time: WAIT_TIME,
        }
    }

    #[allow(dead_code)]
    pub fn new_with_wait_time(
        job: T,
        receiver: mpsc::Receiver<JobRecurserMessage>,
        period: u128,
        wait_time: u64,
    ) -> JobRecurser<T> {
        let next_ts = current_time_in_millis() + period;
        JobRecurser {
            job,
            receiver,
            period,
            next_ts,
            wait_time,
        }
    }

    fn update_next_ts(&mut self) {
        self.next_ts += self.period;
    }

    fn wait(&self, current_time: u128) {
        if current_time + (WAIT_TIME as u128) < self.next_ts {
            sleep(Duration::from_millis(WAIT_TIME));
        } else if current_time < self.next_ts {
            sleep(Duration::from_millis((self.next_ts - current_time) as u64));
        }
    }

    pub fn init(&mut self) {
        loop {
            let message = self.receiver.try_recv();
            if let Ok(JobRecurserMessage::Terminate) = message {
                break;
            }
            let current_time = current_time_in_millis();
            if self.next_ts <= current_time {
                match self.job.execute_job() {
                    Ok(_) => {
                        self.update_next_ts();
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
            self.wait(current_time);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::job_recurser_service::recurser::{JobRecurser, JobRecurserMessage};
    use crate::job_recurser_service::{RecurringJob, RecurringJobError};
    use std::sync::mpsc;
    use std::time::Instant;

    pub struct Job1<'a> {
        qty_executed: &'a mut usize,
        sender: mpsc::Sender<JobRecurserMessage>,
    }

    impl<'a> Job1<'a> {
        pub fn new(qty_executed: &'a mut usize, sender: mpsc::Sender<JobRecurserMessage>) -> Job1 {
            Job1 {
                qty_executed,
                sender,
            }
        }
    }

    impl<'a> RecurringJob for Job1<'a> {
        fn execute_job(&mut self) -> Result<(), RecurringJobError> {
            *self.qty_executed += 1;
            if *self.qty_executed >= 5 {
                self.sender
                    .send(JobRecurserMessage::Terminate)
                    .map_err(|_| RecurringJobError {})?
            }
            Ok(())
        }
    }

    #[test]
    pub fn job_recurser_does_the_job_when_period_is_less_than_wait_time() {
        let (tx, rx) = mpsc::channel();
        let mut qty = 0;
        let job = Job1::new(&mut qty, tx);
        let mut recurser = JobRecurser::new(job, rx, 2);
        let start = Instant::now();
        recurser.init();
        assert_eq!(qty, 5);
        assert!(start.elapsed().as_millis() < 100);
    }

    #[test]
    pub fn job_recurser_does_the_job_when_period_is_greater_than_wait_time() {
        let (tx, rx) = mpsc::channel();
        let mut qty = 0;
        let job = Job1::new(&mut qty, tx);
        let mut recurser = JobRecurser::new_with_wait_time(job, rx, 4, 1);
        let start = Instant::now();
        recurser.init();
        assert_eq!(qty, 5);
        assert!(start.elapsed().as_millis() < 100);
    }
}
