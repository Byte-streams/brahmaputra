use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Brahmaputra{
    pub brahmaputra: BrahmaputraConfig
}

#[derive(Debug, Deserialize)]
pub struct BrahmaputraConfig {
    pub host: String,
    pub brokers: String,
    pub etcd: Etcd,
    pub broker_config: BrokerConfig,
}

#[derive(Debug, Deserialize)]
pub struct Etcd{
    pub connect: String,
}

#[derive(Debug, Deserialize)]
pub struct BrokerConfig{
    pub offsets_topic_replication_factor: u32,
    pub transaction_state_log_replication_factor: u32,
    pub auto_leader_rebalance_enable: bool,
    pub leader_imbalance_check_interval_seconds: i32,
    pub auto_create_topics_enable: bool,
    pub compression_type: String,
    pub delete_topic_enable: bool,
    pub log_flush_interval_messages: u64,
    pub log_flush_interval_ms: u64,
    pub min_insync_replicas: u32,
    pub request_timeout_ms: u64,
    pub max_connections: u32,
    pub num_partitions: u32,
    pub log_dir: String,
    pub socket_send_buffer_bytes: u64,
    pub socket_receive_buffer_bytes: u64,
    pub socket_request_max_bytes: u64,
    pub log_retention_hours: u32,
    pub log_segment_bytes: u64,
    pub log_retention_check_interval_ms: u64
}