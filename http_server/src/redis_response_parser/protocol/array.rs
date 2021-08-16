use crate::redis_response_parser::protocol::DisplayRedisResponse;

pub struct ArrayResponse {
    values: Vec<Box<dyn DisplayRedisResponse>>,
}

impl ArrayResponse {
    pub fn new(values: Vec<Box<dyn DisplayRedisResponse>>) -> Self {
        ArrayResponse { values }
    }
}
impl DisplayRedisResponse for ArrayResponse {
    pub fn to_client_string(&self) -> String {
        let mut result = "".to_owned();
        for (i, value) in self.values.iter().enumerate() {
            result.push_str(&format!("{}) {}", i, value.to_client_string()));
        }
        result
    }
}
