use gateway_core::gateway::publisher::Channel;
use local::device_auth::keystore::KeyManager;
use local::mqtt_connectivity::mqtt_client;
use local::types::config::Config;

use std::fs::File;
use std::sync::{Arc, Mutex};

use iota_streams::app::transport::tangle::client::SendTrytesOptions;

#[tokio::main]
async fn main() -> () {
    //read configuration file
    let config: Config = serde_json::from_reader(File::open("config.json").unwrap()).unwrap();

    let store = KeyManager::new(config.whitelisted_device_ids);

    println!("Starting....");

    let mut send_opt = SendTrytesOptions::default();
    send_opt.min_weight_magnitude = config.mwm;
    send_opt.local_pow = config.local_pow;

    let channel = Arc::new(Mutex::new(Channel::new(config.node, send_opt, None)));
    let addr = channel.lock().expect("").open().unwrap();
    println!("Channel root: {:?}", addr);
    println!("\n To read the messages copy the channel root into https://explorer.iot2tangle.io/ \n ");

    let store = Arc::new(Mutex::new(store));

    mqtt_client::start(
        config.username,
        config.password,
        config.broker_ip,
        config.broker_port,
        config.topic,
        channel,
        store,
    )
    .await
}
