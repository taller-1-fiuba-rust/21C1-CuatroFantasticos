use std::collections::HashMap;
use std::fs;

pub struct Configuration {
    conf: HashMap<String, ConfValue>,
}

pub enum ConfValue {
    Usize(usize),
    String(String),
}

impl Configuration {
    pub fn new(filename: &str) -> Configuration {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let mut conf: HashMap<String, ConfValue> = HashMap::new();
        for line in contents.lines() {
            let split = line.split(':');
            let parsed_line: Vec<&str> = split.collect();
            let usize_fields: Vec<&str> = Vec::from(["verbose", "port", "timeout"]);
            let value;
            if usize_fields.contains(&parsed_line[0]) {
                value = ConfValue::Usize(parsed_line[1].trim().parse().unwrap());
            } else {
                value = ConfValue::String(parsed_line[1].to_owned());
            };
            conf.insert(parsed_line[0].to_owned(), value);
        }
        Configuration { conf }
    }

    pub fn get(&self, key: &str) -> &ConfValue {
        self.conf.get(key).unwrap()
    }
}
