use crate::redis_response_parser::protocol::DisplayRedisResponse;

pub struct SimpleStringResponse {
    value: String,
}

impl SimpleStringResponse {
    pub fn new(value: String) -> Self {
        SimpleStringResponse { value }
    }
}

impl DisplayRedisResponse for SimpleStringResponse {
    fn to_client_string(&self) -> String {
        format!("{}\n", self.value)
    }
}
