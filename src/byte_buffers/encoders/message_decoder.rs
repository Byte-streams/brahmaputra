use crate::byte_buffers::concrete_functions::byte_buffer::ByteBuff;
use crate::byte_buffers::concrete_functions::enums::ExceptionCode;
use crate::byte_buffers::encoders::producer_v1::ProducerMsgV1;
use crate::byte_buffers::encoders::producers::ProducerClass;
use crate::config::configuration::SocketsList;


pub async fn remove_socket_client(client_id: String){
    println!("cleaning socket objects...");
    SocketsList.remove(client_id.as_str()).is_some();
}

pub async fn decode_msg(producer_id: String, total_buf: Option<Box<Vec<u8>>>, error_code: i32){

    // if producer is disconnected
    if error_code == ExceptionCode::DisconnectClient as i32{
        remove_socket_client(producer_id).await;
        return;
    }

    let mut bb = Box::new(ByteBuff{
        multiplier: 10000.0,
        endian: "big".to_string(),
        ..Default::default()
    });

    let temp_buff = total_buf.unwrap();

    bb.wrap(temp_buff.to_vec());

    // deallocating the box
    drop(temp_buff);

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
                    let producer_obj = ProducerMsgV1{};
                    producer_obj.decompress_producer_msg(producer_id, topic, bb).await;
                }
                "C" => {

                }
                _ => {}
            }
        }
        _ => {}
    }
}