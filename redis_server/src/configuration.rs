use std::collections::HashMap;
use std::fs;

pub mod verbose;

pub struct Configuration {
    conf: HashMap<String, String>,
}

impl Configuration {
    pub fn new(filename: &str) -> Configuration {
        let mut conf = Configuration::default_values();
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        for line in contents.lines() {
            let split = line.split(':');
            let parsed_line: Vec<&str> = split.collect();
            conf.insert(parsed_line[0].to_owned(), parsed_line[1].trim().to_owned());
        }
        println!("{:?}", conf);
        Configuration { conf }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.conf.get(key)
    }

    pub fn default_values() -> HashMap<String, String> {
        let mut conf: HashMap<String, String> = HashMap::new();
        conf.insert(String::from("verbose"), String::from("0"));
        conf.insert(String::from("timeout"), String::from("0"));
        conf.insert(String::from("bdfilename"), String::from("dump.rdb"));
        conf.insert(String::from("lofgile"), String::from("logs.txt"));
        conf
    }
}
