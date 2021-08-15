use crate::redis_response_parser::protocol::DisplayRedisResponse;

pub struct ArrayResponse<T: DisplayRedisResponse> {
    values: Vec<T>,
}

impl<T: DisplayRedisResponse> ArrayResponse<T> {
    pub fn new(values: Vec<T>) -> Self {
        ArrayResponse { values }
    }
    pub fn to_client_string(&self) -> String {
        let mut result = "".to_owned();
        for (i, value) in self.values.iter().enumerate() {
            result.push_str(&format!("{}) {}", i, value.to_client_string()));
        }
        result
    }
}
