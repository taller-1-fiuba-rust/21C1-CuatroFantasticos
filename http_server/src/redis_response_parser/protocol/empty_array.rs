use crate::redis_response_parser::protocol::DisplayRedisResponse;

#[derive(Default)]
pub struct EmptyArrayResponse {}

impl EmptyArrayResponse {
    pub fn new() -> Self {
        Default::default()
    }
}
impl DisplayRedisResponse for EmptyArrayResponse {
    fn to_client_string(&self) -> String {
        "(empty list or set)".to_owned()
    }
}
