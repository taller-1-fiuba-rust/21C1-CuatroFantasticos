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

    pub fn rpop(&mut self, times: i32) -> Vec<String> {
        let mut values = Vec::new();
        for _ in 0..times {
            if !self.contents.is_empty() {
                values.push(self.contents.pop().unwrap());
            }
        }
        values
    }
    pub fn rpush(&mut self, new_values: Vec<String>) -> i32 {
        for new_value in new_values {
            self.contents.push(new_value);
        }
        length(self.contents.clone());
    }

    pub fn newRpush(&mut self, new_values: Vec<String>) -> Vec<String> {
        let mut values = Vec::new();
        for value in  new_values {
            values.push(value);

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
    pub fn new(contents_string: String) -> RedisValueList {
        let mut contents = Vec::new();
        let split = contents_string.split(',');
        let parsed_line: Vec<&str> = split.collect();
        for value in parsed_line {
            contents.push(value.trim().to_owned());
        }
        RedisValueList { contents }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::redis_value::list::RedisValueList;

    #[test]
    fn test_create_empty_redis_value() {
        let string = String::from("");
        let redis_value_list = RedisValueList::new(string.clone());
        assert_eq!(redis_value_list.serialize(), string);
    }

    #[test]
    fn test_create_redis_value() {
        let string = String::from("hola, como, estas, ?");
        let redis_value_set = RedisValueList::new(string.clone());
        assert_eq!(redis_value_set.serialize(), string);
    }
}
