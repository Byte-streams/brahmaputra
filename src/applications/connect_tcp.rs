use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::{RwLock, Semaphore};
use uuid::Uuid;
use crate::byte_buffers::concrete_functions::byte_buffer::ByteBuff;
use crate::byte_buffers::encoders::producers::producer_decode_msg;
use crate::config::configuration::{app_conf, SocketsList};

pub async fn connect_tcp_server() -> bool {

    // url of tcp server
    let tcp_url = app_conf.read().await.as_ref().unwrap().brahmaputra.host.to_string();

    println!("Connecting to TCP server at {:?}", tcp_url);

    // start tcp server
    match TcpListener::bind(tcp_url).await {
        Ok(listener) => {

            // semaphore
            let semaphore = Arc::new(Semaphore::new(650000));

            loop {

                let permit = semaphore.clone().acquire_owned().await.unwrap();

                match listener.accept().await {
                    Ok((mut socket, _)) => {
                        let socket_lck = Arc::new(RwLock::new(Some(socket)));
                        let producer_id = Uuid::new_v4().to_string();

                        // Insert the socket into the global SocketsList
                        SocketsList.insert(producer_id.to_string(), Arc::clone(&socket_lck));

                        // Set TCP_NODELAY to true
                        {
                            let mut socket_guard = socket_lck.write().await;
                            if let Some(ref mut socket) = *socket_guard {
                                socket.set_nodelay(true).unwrap();
                            }
                        }

                        // Spawn a new task to handle the connection
                        tokio::spawn(async move {
                            loop {
                                // Buffer to store the length (8 bytes)
                                let mut length_buf = [0u8; 8];

                                // Read exactly 8 bytes from the socket for the length
                                {
                                    let mut socket_guard = socket_lck.write().await;
                                    if let Some(ref mut socket) = *socket_guard {
                                        match socket.read_exact(&mut length_buf).await {
                                            Ok(_) => {
                                                // Convert the 8-byte array to a usize
                                                let total_msg_length = usize::from_be_bytes(length_buf);

                                                if total_msg_length > 0 {
                                                    // Create a buffer for the remaining part of the message
                                                    let mut total_buf = vec![0u8; total_msg_length];

                                                    // Read the exact message length data into the buffer
                                                    match socket.read_exact(&mut total_buf).await {
                                                        Ok(_) => {
                                                            println!("{:?}", total_buf);

                                                            // Process the full message
                                                            producer_decode_msg(producer_id.to_string(), total_buf).await;
                                                        }
                                                        Err(err) => {
                                                            println!("Error reading message data: {}", err);
                                                            remove_socket_client(producer_id);
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                            Err(err) => {
                                                println!("Error reading message length: {}", err);
                                                remove_socket_client(producer_id);
                                                break;
                                            }
                                        }
                                    } else {
                                        println!("Socket has been closed.");
                                        remove_socket_client(producer_id);
                                        break;
                                    }
                                }
                            }
                            drop(permit);
                        });
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

fn remove_socket_client(producer_id: String){
    println!("cleaning socket objects...");
    SocketsList.remove(producer_id.as_str()).is_some();
}