use crate::redis_pattern::RedisPattern;
use std::collections::HashMap;
use std::fs;

pub mod service;
pub mod verbose;

#[derive(Debug, Clone)]
pub struct Configuration {
    conf: HashMap<String, String>,
}

const CONST_VERBOSE: &str = "0";
const CONST_TIMEOUT: &str = "0";
const CONST_DBFILENAME: &str = "dump.rdb";
const CONST_LOGFILE: &str = "logs.txt";

impl Configuration {
    pub fn new(filename: &str) -> Configuration {
        let mut conf = Configuration::default_values();
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        for line in contents.lines() {
            let split = line.split(':');
            let parsed_line: Vec<&str> = split.collect();
            conf.insert(parsed_line[0].to_owned(), parsed_line[1].trim().to_owned());
        }
        Configuration { conf }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.conf.get(key)
    }

    pub fn values_by_pattern(&mut self, pattern: &str) -> Vec<String> {
        let mut matching_keys_values = Vec::new();
        let regex = match RedisPattern::new(pattern) {
            Ok(v) => v,
            Err(_) => return matching_keys_values,
        };
        for (key, value) in self.conf.iter() {
            if regex.is_match(key) {
                matching_keys_values.push(key.to_owned());
                matching_keys_values.push(value.to_owned());
            }
        }
        matching_keys_values
    }

    pub fn set(&mut self, key: String, value: String) {
        self.conf.insert(key, value);
    }

    pub fn default_values() -> HashMap<String, String> {
        let mut conf: HashMap<String, String> = HashMap::new();
        conf.insert(String::from("verbose"), String::from(CONST_VERBOSE));
        conf.insert(String::from("timeout"), String::from(CONST_TIMEOUT));
        conf.insert(String::from("dbfilename"), String::from(CONST_DBFILENAME));
        conf.insert(String::from("logfile"), String::from(CONST_LOGFILE));
        conf
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::Configuration;
    use crate::configuration::CONST_DBFILENAME;
    use crate::configuration::CONST_LOGFILE;
    use crate::configuration::CONST_TIMEOUT;
    use crate::configuration::CONST_VERBOSE;

    #[test]
    fn test_get_default_values() {
        let default_values = Configuration::default_values();
        assert_eq!(
            default_values.get("verbose").expect("verbose").to_owned(),
            String::from(CONST_VERBOSE)
        );
        assert_eq!(
            default_values.get("timeout").expect("timeout").to_owned(),
            String::from(CONST_TIMEOUT)
        );
        assert_eq!(
            default_values
                .get("dbfilename")
                .expect("dbfilename")
                .to_owned(),
            String::from(CONST_DBFILENAME)
        );
        assert_eq!(
            default_values.get("logfile").expect("logfile").to_owned(),
            String::from(CONST_LOGFILE)
        );
    }
}
