use gateway_core::gateway::publisher::Channel;
use local::api::stream_server;
use local::security::keystore::KeyManager;
use local::types::config::Config;

use std::fs::File;
use std::sync::{Arc, Mutex};

use iota_streams::app::transport::tangle::client::SendTrytesOptions;

#[tokio::main]
async fn main() -> () {
    //read configuration file
    let config: Config = serde_json::from_reader(File::open("config.json").unwrap()).unwrap();
    let device_name = config.device_name;
    let broker_ip = config.broker_ip;
    let broker_port = config.broker_port;
    let topic = config.topic;
    let node = config.node;
    let mwm = config.mwm;
    let local_pow = config.local_pow;

    let store = KeyManager::new(device_name.to_string());

    println!("Starting....");

    let mut send_opt = SendTrytesOptions::default();
    send_opt.min_weight_magnitude = mwm;
    send_opt.local_pow = local_pow;

    let channel = Arc::new(Mutex::new(Channel::new(node, send_opt, None)));
    let (addr, _) = channel.lock().expect("").open().unwrap();
    println!("Channel root: {:?}", addr);
    println!("\n To read the messages copy the channel root into http://iot2tangle.link/ \n ");

    let store = Arc::new(Mutex::new(store));

    stream_server::start(broker_ip, broker_port, topic, channel, store).await
}
