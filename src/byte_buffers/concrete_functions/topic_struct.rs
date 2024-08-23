use serde::{Deserialize, Serialize};
use crate::byte_buffers::concrete_functions::broker_config::OverrideConfig;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct topic_settings{
    pub partition_size: i32,
    pub topic_name: String,
    pub file_delete_delay_ms: i64,
    pub flush_messages: i64,
    pub flush_ms: i64,
    pub override_config: OverrideConfig
}