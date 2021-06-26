use crate::data::redis_value::RedisValue;
use std::collections::HashSet;

pub struct RedisValueSet {
    contents: HashSet<String>,
}

impl RedisValue for RedisValueSet {
    fn serialize(&self) -> String {
        let mut res = String::new();
        for (idx, value) in self.contents.iter().enumerate() {
            if idx == 0 {
                res.push_str(&value.to_string());
            } else {
                res.push_str(&format!(", {}", value));
            }
        }
        res
    }
}

impl RedisValueSet {
    pub fn new(contents_string: String) -> RedisValueSet {
        let mut contents = HashSet::new();
        let split = contents_string.split(',');
        let parsed_line: Vec<&str> = split.collect();
        for value in parsed_line {
            contents.insert(value.trim().to_owned());
        }
        RedisValueSet { contents }
    }
}
