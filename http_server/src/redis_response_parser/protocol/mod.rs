pub mod array;
pub mod bulk_string;
pub mod empty_array;
pub mod error;
pub mod integer;
pub mod nil;
pub mod simple_string;

/// DisplayRedisResponse
/// Trait that defines a method called to_client_string, used to make all redis Types parseable to a HttpResponse
///
pub trait DisplayRedisResponse {
    fn to_client_string(&self) -> String;
}
