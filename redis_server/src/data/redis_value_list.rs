use crate::data::redis_value::RedisValue;

pub struct RedisValueList {
    contents: Vec<String>,
}

impl RedisValue for RedisValueList {
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

    fn get_type(&self) -> String {
        String::from("List")
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
    use crate::data::redis_value::RedisValue;
    use crate::data::redis_value_list::RedisValueList;

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
