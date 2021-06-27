use std::sync::mpsc;

pub mod dbsize;

pub trait RedisCommand {
    fn execute(&self) -> String;
}
