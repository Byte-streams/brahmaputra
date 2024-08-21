use etcd_client::{Client, Error};
use crate::config::configuration::{app_conf, GLOBAL};

#[derive(Default)]
pub struct EtcdStore {
    pub client: Option<Client>,
}

impl EtcdStore{
    pub async fn etcd_client_connect(&mut self){

        println!("connecting to etcd server for configuration...");

        let etcd_url = app_conf.read().await.as_ref().unwrap().brahmaputra.etcd.connect.to_string();

        self.client = Option::from(Client::connect([etcd_url.as_str()], None).await.unwrap());
    }
}
