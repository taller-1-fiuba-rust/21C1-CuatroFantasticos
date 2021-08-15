use crate::redis_response_parser::protocol::DisplayRedisResponse;

pub struct IntegerResponse {
    value: i64,
}

impl IntegerResponse {
    pub fn new(value: i64) -> Self {
        IntegerResponse { value }
    }
}

impl DisplayRedisResponse for IntegerResponse {
    fn to_client_string(&self) -> String {
        format!("(integer) {}\n", self.value)
    }
}
