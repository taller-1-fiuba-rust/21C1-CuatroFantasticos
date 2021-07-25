use crate::data::storage::service::operator::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;

#[derive(Debug, Clone)]
pub struct RedisValueList {
    contents: Vec<String>,
}

impl RedisValueList {
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
        String::from("List")
    }

    pub fn length(&self) -> usize {
        self.contents.len()
    }

    pub fn get_index(&self, index: i32) -> Option<String> {
        if index >= 0 {
            self.contents.get(index as usize).cloned()
        } else {
            let index = index + self.contents.len() as i32;
            if index >= 0 {
                self.contents.get(index as usize).cloned()
            } else {
                None
            }
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

    pub fn lpop(&mut self, times: i32) -> Vec<String> {
        let mut values = Vec::new();
        for _ in 0..times {
            if !self.contents.is_empty() {
                values.push(self.contents.remove(0))
            }
        }
        values
    }

    pub fn lpush(&mut self, value: String) {
        self.contents.insert(0, value);
    }

    pub fn rpush(&mut self, value: String) {
        self.contents.push(value);
    }

    pub fn rpop(&mut self, times: i32) -> Vec<String> {
        let mut values = Vec::new();
        for _ in 0..times {
            if !self.contents.is_empty() {
                values.push(self.contents.pop().unwrap());
            }
        }
        values
    }
}

impl ProtocolSerializer for RedisValueList {
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

impl RedisValueList {
    pub fn new() -> RedisValueList {
        let contents: Vec<String> = vec![];
        RedisValueList { contents }
    }
    pub fn new_with_contents(contents_string: String) -> RedisValueList {
        let mut contents = Vec::new();
        let split = contents_string.split(',');
        let parsed_line: Vec<&str> = split.collect();
        for value in parsed_line {
            contents.push(value.trim().to_owned());
        }
        RedisValueList { contents }
    }
}

impl Default for RedisValueList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::redis_value::list::RedisValueList;

    #[test]
    fn test_create_empty_redis_value() {
        let string = String::from("");
        let redis_value_list = RedisValueList::new_with_contents(string.clone());
        assert_eq!(redis_value_list.serialize(), string);
    }

    #[test]
    fn test_create_redis_value() {
        let string = String::from("hola, como, estas, ?");
        let redis_value_set = RedisValueList::new_with_contents(string.clone());
        assert_eq!(redis_value_set.serialize(), string);
    }
}
