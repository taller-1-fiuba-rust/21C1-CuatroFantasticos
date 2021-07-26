use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandMSet {
    member_keys: Vec<String>,
    member_values: Vec<String>,
}

impl RedisCommandMSet {
    pub fn new(member_keys: Vec<String>, member_values: Vec<String>) -> RedisCommandMSet {
        RedisCommandMSet {
            member_keys,
            member_values,
        }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!(
            "Executing command MSet with keys: {:?} and values {:?}: ",
            self.member_keys, self.member_values
        ));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::MSet(
                self.member_keys.clone(),
                self.member_values.clone(),
            ))?;
        let response = response.get_value().protocol_serialize_to_simple_string();
        verbose.print("Finalizing execution of command MSet");
        Ok(response)
    }
}
