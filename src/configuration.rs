use std::fs;

pub struct Configuration {
    verbose: usize,
    port: usize,
    timeout: usize,
    dbfilename: String,
    logfile: String,
}

impl Configuration {
    pub fn new(filename: String) -> Configuration {
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        // TODO: pasar del contents a los atributos del conf
        Configuration {verbose, port, timeout, dbfilename, logfile}
    }
}
