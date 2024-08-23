use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use crate::byte_buffers::concrete_functions::byte_buffer::ByteBuff;
use crate::byte_buffers::concrete_functions::producers_objects::ProducerMsg;
use crate::byte_buffers::encoders::producer_v1::ProducerMsgV1;
use crate::config::configuration::SocketsList;

#[async_trait]
pub trait ProducerClass{
    async fn decompress_producer_msg(&self, producer_id: String, topic: String, bb: Box<ByteBuff>);
    async fn push_producer_message(&self, producer_id: String, topic: String, bb: Box<ByteBuff>);
    async fn error_msg_producer(&self, topic: String, producer: &ProducerMsg) -> Vec<u8>;
}

pub async fn no_topic_error(topic: String, producer: &ProducerMsg, producer_id: String){

    let producer_obj = ProducerMsgV1{};

    // throw error to producer
    let msg = producer_obj.error_msg_producer(topic.to_string(), &producer).await;

    if let Some(ref mut socket) = SocketsList.get(&producer_id){

        let mut socket_guard = socket.write().await;

        if let Some(socket) = &mut *socket_guard {

            let (_, mut writer) = socket.split();

            match writer.write_all(&msg.as_slice()).await{
                Ok(_) => {
                    match writer.flush().await{
                        Err(err) => {
                            println!("{}", err);
                        }
                        _ => {}
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }

        }
    }
}