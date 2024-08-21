#![recursion_limit = "1024"]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod config;
mod applications;
mod byte_buffers;

use std::io;
use libc::{rlimit, RLIMIT_NOFILE, setrlimit};
use clap::Parser;
use crate::applications::connect_etcd::EtcdStore;
use crate::applications::connect_tcp::connect_tcp_server;
use crate::config::read_config::read_config;

// parsing command line argument
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// environment
    #[clap(short, long, default_value = "./config.yaml")]
    env: String,
}

fn set_file_limit(soft_limit: u64, hard_limit: u64) -> io::Result<()> {
    let rlim = rlimit {
        rlim_cur: soft_limit,
        rlim_max: hard_limit,
    };

    let res = unsafe { setrlimit(RLIMIT_NOFILE, &rlim) };

    if res == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {

    // let io_runtime = tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(4)
    //     .enable_io()
    //     .build()
    //     .unwrap();
    //
    // let compute_runtime = tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(4)
    //     .enable_time()
    //     .build()
    //     .unwrap();

    // getting the command line argument
    let args = Args::parse();

    // reading config file
    println!("Reading --env from command line...");

    // get own machine ip
    config::configuration::GLOBAL::get_machine_ip().await;

    // Attempt to set file descriptor limit to 64,000
    set_file_limit(4000, 8000).is_ok();

    // Now you can check the limit using a Rust equivalent of `ulimit -n`
    let mut rlim = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };

    unsafe {
        libc::getrlimit(RLIMIT_NOFILE, &mut rlim);
    }

    println!("Current soft limit: {}", rlim.rlim_cur);
    println!("Current hard limit: {}", rlim.rlim_max);

    // reading the broker config file
    read_config(args.env).await;

    // connect to etcd server
    let mut etcd_obj = EtcdStore{ client: None };
    etcd_obj.etcd_client_connect().await;

    // create a tcp server with tokio
    connect_tcp_server().await;

    // create an actix web server
    // for topic creation and other analytics related to topics, brokers
}
