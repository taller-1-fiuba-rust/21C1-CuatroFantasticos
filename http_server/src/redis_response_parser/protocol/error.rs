use crate::redis_response_parser::protocol::DisplayRedisResponse;

pub struct ErrorResponse {
    value: String,
}

impl ErrorResponse {
    pub fn new(value: String) -> Self {
        ErrorResponse { value }
    }
}

impl DisplayRedisResponse for ErrorResponse {
    fn to_client_string(&self) -> String {
        format!("(error) {}\n", self.value)
    }
}
