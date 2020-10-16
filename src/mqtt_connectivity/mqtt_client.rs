use crate::device_auth::keystore::KeyManager;
use crate::mqtt_connectivity::handlers::handle_sensor_data;
use gateway_core::gateway::publisher::Channel;

extern crate paho_mqtt as mqtt;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{process, thread};

const TOPIC: &'static str = "iot2tangle";

fn on_connect_failure(_cli: &mqtt::AsyncClient, _msgid: u16, rc: i32) {
    println!("Connection attempt failed with error code {}.\n", rc);
    thread::sleep(Duration::from_millis(2500));
}

fn on_connect_success(cli: &mqtt::AsyncClient, _msgid: u16) {
    println!("Connection succeeded");
    cli.subscribe(TOPIC, 1);
}

///
/// Starts the server on the provided port, the server will hand over requests to the handler functions
///
pub async fn start(
    username: String,
    password: String,
    broker_ip: String,
    broker_port: u16,
    _topic: String,
    channel: Arc<Mutex<Channel>>,
    keystore: Arc<Mutex<KeyManager>>,
) -> () {
    let mut state = State::new(channel, keystore);

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(format!("{}:{}", broker_ip, broker_port))
        .client_id("rust_async_subscribe")
        .finalize();

    // Create the client connection
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    cli.set_connected_callback(|_cli: &mqtt::AsyncClient| {});

    cli.set_connection_lost_callback(|cli: &mqtt::AsyncClient| {
        println!("Connection lost. Attempting reconnect.");
        thread::sleep(Duration::from_millis(2500));
        cli.reconnect_with_callbacks(on_connect_success, on_connect_failure);
    });

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
        .clean_session(true)
        .user_name(username)
        .password(password)
        .finalize();

    // Attach a closure to the client to receive callback
    // on incoming messages.
    cli.set_message_callback(move |_cli, msg| {
        if let Some(msg) = msg {
            let payload_str = msg.payload_str();
            state.handle_data(payload_str.to_string());
        }
    });

    // Make the connection to the broker
    cli.connect_with_callbacks(conn_opts, on_connect_success, on_connect_failure);
    println!(
        "Listening for topic: {} on http://{}:{}",
        TOPIC, broker_ip, broker_port
    );

    // Just wait for incoming messages.
    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}

struct State {
    channel: Arc<Mutex<Channel>>,
    keystore: Arc<Mutex<KeyManager>>,
}

impl State {
    pub fn new(channel: Arc<Mutex<Channel>>, keystore: Arc<Mutex<KeyManager>>) -> Self {
        Self {
            channel: channel,
            keystore: keystore,
        }
    }

    pub fn handle_data(&mut self, data: String) -> () {
        handle_sensor_data(data, &self.channel, &self.keystore);
    }
}
