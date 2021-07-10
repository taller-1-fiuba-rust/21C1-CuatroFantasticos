use crate::protocol_serialization::ProtocolSerializer;
use crate::data::redis_value::list::RedisValueList;

pub mod list;
pub mod set;
pub mod string;

pub trait RedisValue: RedisValueClone + Send + ProtocolSerializer {
    fn serialize(&self) -> String;
    fn get_type(&self) -> String;
    fn as_list(&self) -> Result<RedisValueList,String>;
}

pub trait RedisValueClone {
    fn clone_box(&self) -> Box<dyn RedisValue>;
}

impl<T> RedisValueClone for T
where
    T: 'static + RedisValue + Clone,
{
    fn clone_box(&self) -> Box<dyn RedisValue> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn RedisValue> {
    fn clone(&self) -> Box<dyn RedisValue> {
        self.clone_box()
    }
}
