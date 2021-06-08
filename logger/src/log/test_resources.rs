use std::sync::{Arc, Mutex};
use std::io::Write;

pub struct VectorWriter {
    strings_received: Arc<Mutex<Vec<String>>>,
}

impl VectorWriter {
    pub fn new() -> Self {
        let strings_received = Arc::new(Mutex::new(Vec::new()));
        VectorWriter { strings_received }
    }
    pub fn get_vector_copy(&self) -> Arc<Mutex<Vec<String>>> {
        self.strings_received.clone()
    }
}

impl Write for VectorWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize,std::io::Error> {
        let s = match std::str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        self.strings_received.lock().unwrap().push(s.to_string());
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(),std::io::Error> {
        Ok(())
    }
}