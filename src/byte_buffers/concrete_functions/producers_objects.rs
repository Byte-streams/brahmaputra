use tokio::net::TcpStream;

#[derive(Debug)]
pub struct ProducerMsg {
    pub compression_type: Option<String>,
    pub acks: Option<String>,
    pub partition: i32,
    pub key: String,
    pub msg: Vec<u8>,
    pub topic: String,
}
