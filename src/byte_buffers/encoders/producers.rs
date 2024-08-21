use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::byte_buffers::concrete_functions::byte_buffer::ByteBuff;
use crate::byte_buffers::concrete_functions::enums::MessageCode;
use crate::byte_buffers::concrete_functions::producers_objects::ProducerMsg;
use crate::config::configuration::{app_conf, SocketsList, TopicCreated};

pub async fn producer_decode_msg(producer_id: String, total_buf: Vec<u8>){

    let mut bb = ByteBuff{
        multiplier: 10000.0,
        endian: "big".to_string(),
        ..Default::default()
    };

    bb.wrap(total_buf.to_vec());

    // get the version for now and ignore
    let version = bb.get_string();

    // checking if there is topic
    let topic = bb.get_string();

    // check if producer or consumer
    let client_type = bb.get_string();

    match version.as_str(){
        "V_1" => {
            match client_type.as_str(){
                "P" => {
                    decompress_producer_msg_v1(producer_id, topic, bb).await;
                }
                "C" => {

                }
                _ => {}
            }
        }
        _ => {}
    }
}

async fn decompress_producer_msg_v1(producer_id: String, topic: String, mut bb: ByteBuff){

    // producer message type
    let producer_message_type = bb.get_int();

    // enum
    let producer_msg = MessageCode::ProducerMsg as i32;

    match producer_message_type{
        producer_msg => {
            push_producer_message_V1(producer_id, topic, bb).await;
        }
        _ => {}
    }
}

async fn push_producer_message_V1(producer_id: String, topic: String, mut bb: ByteBuff){

    let mut producer = ProducerMsg{
        compression_type: None,
        acks: None,
        partition: 0,
        key: "".to_string(),
        msg: vec![],
        topic,
    };

    // if there is any compression
    producer.compression_type = Some(bb.get_string());

    // if there is any acks
    producer.acks = Some(bb.get_string());

    // getting the partition
    producer.partition = bb.get_int();

    // getting the key
    producer.key = bb.get_string();

    // if there is message
    producer.msg = bb.get();

    // checking if there is topic created if there is no topic created check for auto_create_topics_enable - true
    // if auto_create_topics_enable - true then create topic and put it into etcd
    let auto_create_topics_enable = app_conf.read().await.as_ref().unwrap().brahmaputra.broker_config.auto_create_topics_enable;

    // checking if topic is not created and auto_create_topics_enable is also false
    if !auto_create_topics_enable && !TopicCreated.contains_key(producer.topic.as_str()){
        // throw error to producer

    }

    if auto_create_topics_enable && !TopicCreated.contains_key(producer.topic.as_str()){

    }

    // push to channel to write into file
    println!("{:?}", producer);
}

pub async fn push_error_msg_producer(){

}