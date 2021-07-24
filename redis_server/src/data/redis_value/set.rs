use crate::data::storage::service::operator::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct RedisValueSet {
    contents: HashSet<String>,
}

impl RedisValueSet {
    pub fn serialize(&self) -> String {
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

    pub fn get_type(&self) -> String {
        String::from("Set")
    }

    pub fn add(&mut self, member: String) -> i32 {
        let added = self.contents.insert(member);
        if added {
            1
        } else {
            0
        }
    }

    pub fn sort(&self) -> Result<Vec<String>, RedisError> {
        let mut contents: Vec<i32> = vec![];
        for x in &self.contents {
            match x.parse::<i32>() {
                Err(_) => return Err(RedisError::NotASetOfNumbers),
                Ok(value) => contents.push(value),
            }
        }
        contents.sort_unstable();
        let sorted = contents.iter().map(|v| v.to_string()).collect();
        Ok(sorted)
    }

    pub fn length(&self) -> usize {
        self.contents.len()
    }

    pub fn contains(&self, member: String) -> bool {
        self.contents.contains(&member)
    }
}

impl ProtocolSerializer for RedisValueSet {
    fn protocol_serialize_to_simple_string(&self) -> String {
        todo!()
    }

    fn protocol_serialize_to_int(&self) -> String {
        todo!()
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        todo!()
    }
}

impl RedisValueSet {
    pub fn new_with_contents(contents_string: String) -> RedisValueSet {
        let mut contents = HashSet::new();
        let split = contents_string.split(',');
        for value in split {
            contents.insert(value.trim().to_string());
        }
        RedisValueSet { contents }
    }

    pub fn new() -> RedisValueSet {
        let contents = HashSet::<String>::new();
        RedisValueSet { contents }
    }
}

impl Default for RedisValueSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::redis_value::set::RedisValueSet;

    #[test]
    fn test_create_empty_redis_value() {
        let string = String::from("");
        let redis_value_set = RedisValueSet::new_with_contents(string.clone());
        assert_eq!(redis_value_set.serialize(), string);
    }

    #[test]
    fn test_create_redis_value() {
        let string = String::from("hola");
        let redis_value_set = RedisValueSet::new_with_contents(string.clone());
        assert_eq!(redis_value_set.serialize(), string);
    }
}
