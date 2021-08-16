use crate::redis_response_parser::protocol::DisplayRedisResponse;

pub struct BulkStringResponse {
    value: String,
}

impl BulkStringResponse {
    pub fn new(value: String) -> Self {
        BulkStringResponse { value }
    }
}

impl DisplayRedisResponse for BulkStringResponse {
    fn to_client_string(&self) -> String {
        format!("\"{}\"", self.value)
    }
}
