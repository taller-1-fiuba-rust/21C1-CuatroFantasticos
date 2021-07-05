pub mod list;
pub mod set;
pub mod string;

pub trait RedisValue {
    fn serialize(&self) -> String;
}
