use crate::data::redis_value::RedisValue;
use crate::data::redis_value_list::RedisValueList;
use crate::data::redis_value_set::RedisValueSet;
use crate::data::redis_value_string::RedisValueString;
use std::collections::HashMap;
use std::fs;

pub struct Storage {
    storage: HashMap<String, Box<dyn RedisValue>>,
}

impl Storage {
    pub fn new(filename: &str) -> Storage {
        let contents = fs::read_to_string(filename);
        let storage = match contents {
            Ok(contents) => Storage::deserialize(contents),
            Err(_) => Storage::deserialize_empty(),
        };
        Storage { storage }
    }

    pub fn deserialize(contents: String) -> HashMap<String, Box<dyn RedisValue>> {
        let mut storage: HashMap<String, Box<dyn RedisValue>> = HashMap::new();
        for line in contents.lines() {
            let split = line.split(':');
            let parsed_line: Vec<&str> = split.collect();
            match parsed_line[1].trim() {
                "string" => {
                    let value = RedisValueString::new(parsed_line[2].trim().to_owned());
                    storage.insert(parsed_line[0].trim().to_owned(), Box::new(value));
                }
                "list" => {
                    let value = RedisValueList::new(parsed_line[2].trim().to_owned());
                    storage.insert(parsed_line[0].trim().to_owned(), Box::new(value));
                }
                "hash" => {
                    let value = RedisValueSet::new(parsed_line[2].trim().to_owned());
                    storage.insert(parsed_line[0].trim().to_owned(), Box::new(value));
                }
                _ => println!("aun no implementado"),
            }
        }
        storage
    }

    pub fn deserialize_empty() -> HashMap<String, Box<dyn RedisValue>> {
        let storage: HashMap<String, Box<dyn RedisValue>> = HashMap::new();
        storage
    }

    pub fn serialize(&self) -> Vec<String> {
        let mut contents = Vec::new();
        for (key, value) in &self.storage {
            let line = format!("{}: {}", key, value.serialize());
            contents.push(line);
        }
        contents
    }

    pub fn imprimir(&self) {
        for (key, value) in &self.storage {
            println!("{:?}", key);
            println!("{:?}", value.serialize());
        }
    }
}
