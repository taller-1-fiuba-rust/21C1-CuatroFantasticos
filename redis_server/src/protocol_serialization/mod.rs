pub trait ProtocolSerializer {
    fn protocol_serialize_to_simple_string(&self) -> String;
    fn protocol_serialize_to_int(&self) -> String;
    fn protocol_serialize_to_bulk_string(&self) -> String;
}

impl ProtocolSerializer for String {
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

impl ProtocolSerializer for &str {
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

impl ProtocolSerializer for Vec<String> {
    fn protocol_serialize_to_simple_string(&self) -> String {
        let len = self.len();
        let mut response = format!("*{}\r\n", len);
        for x in self.iter() {
            response.push_str(&x.protocol_serialize_to_bulk_string());
        }
        response
    }

    fn protocol_serialize_to_int(&self) -> String {
        self.protocol_serialize_to_simple_string()
    }

    fn protocol_serialize_to_bulk_string(&self) -> String {
        self.protocol_serialize_to_simple_string()
    }
}
