use crate::configuration::verbose::Verbose;
use logger::log::logger::Logger;
use std::collections::HashMap;
use std::fs;
use std::sync::mpsc;
use crate::data::redis_request::RedisRequest;

pub mod verbose;

#[derive(Debug, Clone)]
pub struct Configuration {
    conf: HashMap<String, String>,
    logger: Option<Logger>,
    verbose: Verbose,
    data_sender: Option<mpsc::Sender<RedisRequest>>,
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
        let verbose =
            Configuration::create_verbose(conf.get("verbose").expect("No hay un verbose definido"));
        Configuration {
            conf,
            verbose,
            logger: None,
            data_sender: None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.conf.get(key)
    }

    pub fn default_values() -> HashMap<String, String> {
        let mut conf: HashMap<String, String> = HashMap::new();
        conf.insert(String::from("verbose"), String::from(CONST_VERBOSE));
        conf.insert(String::from("timeout"), String::from(CONST_TIMEOUT));
        conf.insert(String::from("dbfilename"), String::from(CONST_DBFILENAME));
        conf.insert(String::from("logfile"), String::from(CONST_LOGFILE));
        conf
    }

    fn create_verbose(verbose: &str) -> Verbose {
        Verbose::new(verbose)
    }
    pub fn set_logservice(&mut self, logger: Logger) {
        self.logger = Some(logger);
    }

    pub fn set_data_sender(&mut self, data_sender: mpsc::Sender<RedisRequest>){
        self.data_sender = Some(data_sender);
    }

    pub fn get_data_sender(&mut self) -> &mpsc::Sender<RedisRequest>{
        self.data_sender.as_ref().expect("No sender in configuration")
    }

    pub fn create_logger(&self) -> Logger {
        self.logger.as_ref().expect("no hay un logservice").clone()
    }

    pub fn verbose(&self, content: &str) {
        self.verbose.print(content);
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
