use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Returns the set cardinality (number of elements) of the set stored at key.
/// # Arguments
/// * key - String
/// * number - String
///
///# Return value
/// Integer reply, specifically:
/// * 1 if the element is a member of the set.
/// * 0 if the element is not a member of the set, or if key does not exist.
///
///Examples
/// ```redis> SADD myset "one"
/// (integer) 1
/// redis> SISMEMBER myset "one"
/// (integer) 1
/// redis> SISMEMBER myset "two"
/// (integer) 0
/// redis>
/// ```
pub struct RedisCommandSismember {
    key: String,
    member: String,
}

impl RedisCommandSismember {
    pub fn new(key: String, member: String) -> RedisCommandSismember {
        RedisCommandSismember { key, member }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command Sismember with key: {} and member : {} ",
            self.key, self.member
        ));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Sismember(
                self.key.clone(),
                self.member.clone(),
            ))?;
        let response = response.get_value().protocol_serialize_to_int();
        verbose.print("Finalizing execution of command Sismember");
        Ok(response)
    }
}
