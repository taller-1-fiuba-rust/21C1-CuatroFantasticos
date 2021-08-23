pub mod array;
pub mod bulk_string;
pub mod empty_array;
pub mod error;
pub mod integer;
pub mod nil;
pub mod simple_string;

pub trait DisplayRedisResponse {
    fn to_client_string(&self) -> String;
}
