pub trait ProtocolSerializer {
    fn protocol_serialize_to_simple_string(&self) -> String;
    fn protocol_serialize_to_int(&self) -> String;
    fn protocol_serialize_to_bulk_string(&self) -> String;
}

impl<T: ToString> ProtocolSerializer for T {
    fn protocol_serialize_to_simple_string(&self) -> String {
        format!("+{}\r\n", self.to_string())
    }

    fn protocol_serialize_to_int(&self) -> String {
        format!(":{}\r\n", self.to_string())
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        format!("${}\r\n{}\r\n", self.to_string().len(), self.to_string())
    }
}
