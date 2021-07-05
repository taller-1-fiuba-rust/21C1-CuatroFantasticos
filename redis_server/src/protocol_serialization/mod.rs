pub trait ProtocolSerializer {
    fn protocol_serialize_to_simple_string(&self) -> String;
    fn protocol_serialize_to_int(&self) -> String;
    fn protocol_serialize_to_bulk_string(&self) -> String;
}

impl ProtocolSerializer for &str {
    fn protocol_serialize_to_simple_string(&self) -> String {
        format!("+{}\r\n", self)
    }

    fn protocol_serialize_to_int(&self) -> String {
        format!(":{}\r\n", self)
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        format!("${}\r\n{}\r\n", self.len(), self)
    }
}
