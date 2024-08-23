use std::fs::File;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, ToSocketAddrs};
use std::process::Command;
use std::ptr::addr_of;
use std::sync::Arc;
use dashmap::DashMap;
use lazy_static::lazy_static;
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use crate::byte_buffers::concrete_functions::broker_config::Brahmaputra;

// declaring static mutable variable for containing the config object
lazy_static! {
    pub static ref SYS_IP: RwLock<Option<String>> = RwLock::new(None);
    pub static ref app_conf: RwLock<Option<Brahmaputra>> = RwLock::new(None);
    pub static ref SocketsList:DashMap<String, Arc<RwLock<Option<TcpStream>>>> = DashMap::with_shard_amount(32);
    pub static ref TopicCreated:DashMap<String, bool> = DashMap::with_shard_amount(32);
    pub static ref ThreadModel: Runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(100000)
        .enable_all()
        .build()
        .unwrap();
}

// creating a global config struct
#[derive(Clone, Debug)]
pub struct GLOBAL {}

// adding drop traits
impl Drop for GLOBAL {
    fn drop(&mut self) {
        println!("Dropping config object...");
    }
}

// implementing the global struct adding read_config and get_config methods
impl GLOBAL {
    // setting the config
    pub async fn set_app_config(json_string: String) {
        app_conf.write().await.insert(serde_yml::from_str(json_string.as_str()).unwrap());
    }

    // get machine ip
    pub async fn get_machine_ip() -> String {
        return match GLOBAL::get_host_ip() {
            None => {
                "".to_string()
            }
            Some(val) => {
                let _ = SYS_IP.write().await.insert(val.to_string());
                println!("{:?}", SYS_IP.read().await);
                GLOBAL::get_host_ip().unwrap().to_string()
            }
        };
    }

    // get machine ip
    pub(crate) fn get_host_ip() -> Option<IpAddr> {
        let output = Command::new("hostname")//launch a command on cmd or bash to get the hosts name i.e the machine name e.g rustpc
            .output()
            .expect("failed to execute `hostname`");//generate error if the command fails to execute
        let std_out = String::from_utf8(output.stdout).unwrap();//get the returned output from the outcome of the above command as the host name string
        let std_res = std_out.trim();//trim the output string to get rid of any semicolons etc to just leave the host name
        match GLOBAL::name_to_ip(std_res) {
            Ok(name_to_ip_res) => {
                let host_ip_option = name_to_ip_res.last();//it was found that the host ip adapter socket always shows at the end of the list or is the sole one in the list
                //either way extracting the last value of the list gives us the desired ip address
                let host_ip = host_ip_option.unwrap().to_string();//extract the host ip address from the option wrapping
                let ips: Vec<&str> = host_ip.trim().split(" ").collect::<Vec<&str>>();//generate a vector to test for the ip address type
                let first = ips.first();//generate an option type to work with the match statement
                match first {
                    Some(first) => {
                        if !first.is_empty() {//confirm if no null values are present in the option
                            if let Ok(addr) = first.parse::<Ipv4Addr>() {//if ip type ip v4 return as ip v4 option
                                return Some(IpAddr::V4(addr));
                            } else if let Ok(addr) = first.parse::<Ipv6Addr>() {//if ip type ip v6 return as ip v6 option
                                return Some(IpAddr::V6(addr));
                            } else {
                                None//if the ip type not matched to above values return null value
                            }
                        } else {
                            None//if option empty return null value
                        }
                    }
                    None => None//if null value to be returned that return option with null value
                }
            }
            Err(e) => {
                println!("{:?}", e);
                None
            }
        }
    }

    /*
    * Custom function to get the host name as a string and generate a vector list of all the socket addresses present on it
    */
    fn name_to_ip(host: &str) -> io::Result<Vec<IpAddr>> {
        (host, 0).to_socket_addrs().map(|iter| iter.map(|socket_address| socket_address.ip()).collect())//run a map iterator to generate
        //a vector of the list of all the socket address on the given host name
    }
}
