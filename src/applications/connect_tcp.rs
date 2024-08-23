use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock, RwLockWriteGuard, Semaphore};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use crate::byte_buffers::concrete_functions::byte_buffer::ByteBuff;
use crate::byte_buffers::concrete_functions::enums::ExceptionCode;
use crate::byte_buffers::encoders::message_decoder::{decode_msg, remove_socket_client};
use crate::config::configuration::{app_conf, SocketsList, ThreadModel};

pub async fn connect_tcp_server() -> bool {

    // url of tcp server
    let tcp_url = app_conf.read().await.as_ref().unwrap().brahmaputra.host.to_string();
    let buffered_channel_size = app_conf.read().await.as_ref().unwrap().brahmaputra.broker_config.buffered_channel_size;

    println!("Connecting to TCP server at {:?}", tcp_url);

    // open tokio channels
    // Create an unbounded channel
    let (mut tx, mut rx) = mpsc::channel::<(String, Option<Box<Vec<u8>>>, i32)>(buffered_channel_size as usize);
    let mut tx_arc = Arc::new(RwLock::new(tx));

    // channel receiver thread
    ThreadModel.spawn(async move{
        // Receive the messages
        while let Some((client_id, total_buf, error_code)) = rx.recv().await {
            decode_msg(client_id.to_string(), total_buf, error_code).await;
        }
    });

    // start tcp server
    match TcpListener::bind(tcp_url).await {
        Ok(listener) => {

            // semaphore
            let semaphore = Arc::new(Semaphore::new(650000));

            loop {

                let permit = semaphore.clone().acquire_owned().await.unwrap();

                match listener.accept().await {
                    Ok((mut socket, _)) => {
                        socket.set_nodelay(true).unwrap();
                        let socket_lck = Arc::new(RwLock::new(Some(socket)));
                        let client_id = Uuid::new_v4().to_string();

                        // Insert the socket into the global SocketsList
                        SocketsList.insert(client_id.to_string(), Arc::clone(&socket_lck));

                        let tx_arc_cl = Arc::clone(&tx_arc);

                        read_socket_msg(client_id, tx_arc_cl, socket_lck).await;

                        drop(permit);
                    }
                    Err(err) => {
                        println!("Error accepting connection: {}", err);
                        return false;
                    }
                }
            }
        }
        Err(err) => {
            println!("Error binding to TCP socket: {}", err);
            return false;
        }
    }
}

async fn read_socket_msg(client_id: String, tx_arc_cl: Arc<RwLock<Sender<(String, Option<Box<Vec<u8>>>, i32, )>>>, socket_lck: Arc<RwLock<Option<TcpStream>>>){

    // Spawn a new task to handle the connection
    ThreadModel.spawn(async move {
        loop {
            // Buffer to store the length (8 bytes)
            let mut length_buf = [0u8; 8];

            // Read exactly 8 bytes from the socket for the length
            match socket_lck.write().await.as_mut(){
                None => {}
                Some(sock) => {
                    match sock.read_exact(&mut length_buf).await {
                        Ok(_) => {
                            // Convert the 8-byte array to a usize
                            let total_msg_length = usize::from_be_bytes(length_buf);

                            if total_msg_length > 0 {
                                // Create a buffer for the remaining part of the message
                                let mut total_buf = Box::new(vec![0u8; total_msg_length]);

                                // Read the exact message length data into the buffer
                                match sock.read_exact(&mut total_buf).await {
                                    Ok(_) => {
                                        // Process the full message
                                        write_to_channel(client_id.to_string(), Some(total_buf), ExceptionCode::Ignore as i32, &tx_arc_cl).await;
                                    }
                                    Err(err) => {
                                        println!("Error reading message data: {}", err);
                                        write_to_channel(client_id.to_string(), Some(total_buf), ExceptionCode::DisconnectClient as i32, &tx_arc_cl).await;
                                        break;
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            println!("Error reading message length: {}", err);
                            write_to_channel(client_id.to_string(), None, ExceptionCode::DisconnectClient as i32, &tx_arc_cl).await;
                            break;
                        }
                    }
                }
            }
        }
    });
}

async fn write_to_channel(client_id: String, total_buf: Option<Box<Vec<u8>>>, error_code: i32, tx_arc_cl: &Arc<RwLock<Sender<(String, Option<Box<Vec<u8>>>, i32)>>>){
    match tx_arc_cl.write().await.send((client_id.to_string(), total_buf, error_code)).await{
        Err(err) => {
            println!("Error reading message data: {}", err);
            remove_socket_client(client_id).await;
        }
        _ => {}
    }
}