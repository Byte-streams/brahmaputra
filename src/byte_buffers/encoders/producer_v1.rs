use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use crate::byte_buffers::concrete_functions::byte_buffer::ByteBuff;
use crate::byte_buffers::concrete_functions::enums::{ExceptionCode, MessageCode, ProducerErrorCode};
use crate::byte_buffers::concrete_functions::messages::NO_TOPIC_EXISTS;
use crate::byte_buffers::concrete_functions::producers_objects::ProducerMsg;
use crate::byte_buffers::encoders::producers::{no_topic_error, ProducerClass};
use crate::config::configuration::{app_conf, SocketsList, TopicCreated};

pub struct ProducerMsgV1;

#[async_trait]
impl ProducerClass for ProducerMsgV1{
    async fn decompress_producer_msg(&self, producer_id: String, topic: String, mut bb: Box<ByteBuff>){

        // producer message type
        let producer_message_type = bb.get_int();

        // enum
        let producer_msg = MessageCode::ProducerMsg as i32;

        match producer_message_type{
            producer_msg => {
                self.push_producer_message(producer_id, topic, bb).await;
            }
            _ => {}
        }
    }

    async fn push_producer_message(&self, producer_id: String, topic: String, mut bb: Box<ByteBuff>){

        let mut producer = ProducerMsg{
            compression_type: None,
            acks: None,
            partition: 0,
            unique_key: "".to_string(),
            key: "".to_string(),
            msg: vec![],
            topic: topic.to_string(),
        };

        // if there is any compression
        producer.compression_type = Some(bb.get_string());

        // if there is any acks
        producer.acks = Some(bb.get_string());

        // getting the partition
        producer.partition = bb.get_int();

        // getting the unique key
        producer.unique_key = bb.get_string();

        // getting the key
        producer.key = bb.get_string();

        // if there is message
        producer.msg = bb.get();

        // checking if there is topic created if there is no topic created check for auto_create_topics_enable - true
        // if auto_create_topics_enable - true then create topic and put it into etcd
        let auto_create_topics_enable = app_conf.read().await.as_ref().unwrap().brahmaputra.broker_config.override_config.auto_create_topics_enable;

        // checking if topic is not created and auto_create_topics_enable is also false
        if !auto_create_topics_enable && !TopicCreated.contains_key(producer.topic.as_str()){
            no_topic_error(topic.to_string(), &producer, producer_id.to_string()).await;
            return;
        }

        // checking if auto_create_topics_enable is true but topic does not exist then create a new topic
        // put it into etcd and hashmap and create folder and directory
        if auto_create_topics_enable && !TopicCreated.contains_key(producer.topic.as_str()){



        }

        // push to channel to write into file
        println!("{:?}", producer);

        // Manually deallocate the Box
        drop(bb);
    }

    async fn error_msg_producer(&self, topic: String, producer: &ProducerMsg) -> Vec<u8>{

        let mut bb = ByteBuff{
            multiplier: 10000.0,
            endian: "big".to_string(),
            ..Default::default()
        };

        // putting as P
        bb.put_string("P".to_string());

        // putting Error Code
        bb.put_int(ProducerErrorCode::NoTopicExists as i32);

        // putting error message
        bb.put_string(NO_TOPIC_EXISTS.to_string());

        // putting topic
        bb.put_string(topic);

        // putting partition
        bb.put_int(producer.partition);

        // putting unique key
        bb.put_string(producer.unique_key.to_string());

        // putting key
        bb.put_string(producer.key.to_string());

        // getting total message byte array
        let total_msg = bb.to_array();

        bb = ByteBuff{
            multiplier: 10000.0,
            endian: "big".to_string(),
            ..Default::default()
        };

        // putting it into bb
        bb.put(total_msg);

        // return total byte array
        bb.to_array()
    }
}
