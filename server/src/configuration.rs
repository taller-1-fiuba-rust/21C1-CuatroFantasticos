use std::collections::HashMap;
use std::fs;

pub struct Configuration {
    conf: HashMap<String, ConfValue>,
}

#[derive(Debug)]
pub enum ConfValue {
    Verbose(usize),
    Timeout(usize),
    Port(String),
    Dbfilename(String),
    LogFile(String),
}

impl ConfValue {
    pub fn new(key: &str, value: &str) -> ConfValue {
        match key {
            "verbose" => ConfValue::Verbose(value.trim().parse().unwrap()),
            "timeout" => ConfValue::Timeout(value.trim().parse().unwrap()),
            "port" => ConfValue::Port(value.trim().to_owned()),
            "dbfilename" => ConfValue::Dbfilename(value.trim().to_owned()),
            "logfile" => ConfValue::LogFile(value.trim().to_owned()),
            _ => panic!("{:?}", "ERROR AL PARSEAR CONFIG"),
        }
    }

    pub fn get_port(&self) -> &String {
        match self {
            ConfValue::Port(value) => value,
            _ => panic!("{:?}", "ESTO NO ES PORT"),
        }
    }
}

impl Configuration {
    pub fn new(filename: &str) -> Configuration {
        let mut conf: HashMap<String, ConfValue> = HashMap::new();

        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        for line in contents.lines() {
            let split = line.split(':');
            let parsed_line: Vec<&str> = split.collect();
            let value = ConfValue::new(parsed_line[0], parsed_line[1]);
            conf.insert(parsed_line[0].to_owned(), value);
        }
        println!("{:?}", conf);
        Configuration { conf }
    }

    pub fn get(&self, key: &str) -> &ConfValue {
        self.conf.get(key).unwrap()
    }
}
