use crate::command::RedisCommand;
use std::sync::mpsc;
use crate::data::storage::Storage;

pub struct RedisCommandDbSize {

}

impl RedisCommandDbSize {
    pub fn new(storage: Storage, sender: mpsc::Sender<String>) {

    }
}

impl RedisCommand for RedisCommandDbSize {
    pub fn execute(&self) -> String {

    }
}