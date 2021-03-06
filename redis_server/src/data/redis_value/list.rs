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

    fn index(&self, number: i32) -> Result<usize, ()> {
        let mut idx = number;
        if idx < 0 {
            idx += self.contents.len() as i32;
        }
        if idx >= 0 && idx < self.contents.len() as i32 {
            Ok(idx as usize)
        } else {
            Err(())
        }
    }

    pub fn get_index(&self, index: i32) -> Option<String> {
        match self.index(index) {
            Ok(idx) => self.contents.get(idx).cloned(),
            Err(_) => None,
        }
    }

    pub fn lrange(&self, start: i32, stop: i32) -> Result<Vec<String>, RedisError> {
        match self.index(start) {
            Ok(start_idx) => match self.index(stop) {
                Ok(stop_idx) => match self.contents.get(start_idx..stop_idx + 1) {
                    Some(value) => Ok(value.to_vec()),
                    None => Err(RedisError::IdxOutOfRange),
                },
                Err(_) => Err(RedisError::IdxOutOfRange),
            },
            Err(_) => Err(RedisError::IdxOutOfRange),
        }
    }

    pub fn lrem(&mut self, count: i32, element: String) -> i32 {
        let mut times = count;
        let mut i = 0;
        while i < self.contents.len() {
            match self.contents.get(i) {
                Some(value) => {
                    if *value == element {
                        self.contents.remove(i);
                        times -= 1;
                        if times == 0 {
                            break;
                        }
                    } else {
                        i += 1;
                    }
                }
                None => break,
            }
        }
        count - times
    }

    pub fn rem_all(&mut self, element: String) -> i32 {
        let mut times = 0;
        let mut i = 0;
        while i < self.contents.len() {
            match self.contents.get(i) {
                Some(value) => {
                    if *value == element {
                        self.contents.remove(i);
                        times += 1;
                    } else {
                        i += 1;
                    }
                }
                None => break,
            }
        }
        times
    }

    pub fn rrem(&mut self, count: i32, element: String) -> i32 {
        let mut times = count;
        let mut i = self.contents.len();
        while i > 0 {
            match self.contents.get(i - 1) {
                Some(value) => {
                    if *value == element {
                        self.contents.remove(i - 1);
                        times -= 1;
                        if times == 0 {
                            break;
                        }
                    } else {
                        i -= 1;
                    }
                }
                None => i -= 1,
            }
        }
        count - times
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

    pub fn replace(&mut self, index: i32, value: String) -> bool {
        match self.index(index) {
            Ok(idx) => {
                self.contents.remove(idx);
                self.contents.insert(idx, value);
                true
            }
            Err(_) => false,
        }
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
