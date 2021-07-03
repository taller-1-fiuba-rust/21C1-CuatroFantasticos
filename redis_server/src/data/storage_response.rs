use std::collections::HashSet;

pub struct StorageResponse {
    value_response: StorageResponseEnum,
}

impl StorageResponse {
    pub fn new(value_response: StorageResponseEnum) -> StorageResponse {
        StorageResponse { value_response }
    }

    pub fn get_value(&self) -> StorageResponseEnum {
        self.value_response.clone()
    }
}

#[derive(Clone)]
pub enum StorageResponseEnum {
    ResponseInt(usize),
    ResponseStr(String),
    ResponseList(Vec<String>),
    ResponseSet(HashSet<String>),
    ResponseBool(bool),
    ResponseError(String),
}
