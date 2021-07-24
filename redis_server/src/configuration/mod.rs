use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::sync::mpsc;

use logger::log_service::log_interface::LogInterface;
use logger::log_service::logger::Logger;

use crate::configuration::verbose::Verbose;
use crate::data::storage::service::operator::request_message::StorageRequestMessage;

pub mod service;
pub mod verbose;

#[derive(Debug, Clone)]
pub struct Configuration {
    conf: HashMap<String, String>,
    logger_builder: Option<LogInterface<File>>,
    verbose: Verbose,
    data_sender: Option<mpsc::Sender<StorageRequestMessage>>,
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
            logger_builder: None,
            data_sender: None,
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.conf.get(key).map(|s| s.to_string())
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

    fn create_verbose(verbose: &str) -> Verbose {
        Verbose::new(verbose)
    }
    pub fn set_logger_builder(&mut self, logger_builder: LogInterface<File>) {
        self.logger_builder = Some(logger_builder);
    }

    pub fn set_data_sender(&mut self, data_sender: mpsc::Sender<StorageRequestMessage>) {
        self.data_sender = Some(data_sender);
    }

    pub fn get_data_sender(&mut self) -> &mpsc::Sender<StorageRequestMessage> {
        self.data_sender
            .as_ref()
            .expect("No sender in configuration")
    }

    pub fn logger(&self) -> Logger<File> {
        self.logger_builder
            .as_ref()
            .expect("There is no logger builder set")
            .build_logger()
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
