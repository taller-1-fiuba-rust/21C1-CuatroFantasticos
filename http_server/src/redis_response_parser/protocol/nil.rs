use crate::redis_response_parser::protocol::DisplayRedisResponse;

#[derive(Default)]
pub struct NilResponse {}

impl NilResponse {
    pub fn new() -> Self {
        Default::default()
    }
}
impl DisplayRedisResponse for NilResponse {
    fn to_client_string(&self) -> String {
        "nil".to_owned()
    }
}
