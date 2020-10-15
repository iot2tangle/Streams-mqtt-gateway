use crate::device_auth::keystore::KeyManager;
use crate::mqtt_connectivity::handlers::handle_sensor_data;
use gateway_core::gateway::publisher::Channel;

use rumqtt::{MqttClient, MqttOptions, Notification, QoS, SecurityOptions};
use std::sync::{Arc, Mutex};

///
/// Starts the server on the provided port, the server will hand over requests to the handler functions
///
pub async fn start(
    userame: String,
    password: String,
    broker_ip: String,
    broker_port: u16,
    topic: String,
    channel: Arc<Mutex<Channel>>,
    keystore: Arc<Mutex<KeyManager>>,
) -> () {
    let mqtt_options = MqttOptions::new("iot2tangle", broker_ip.clone(), broker_port)
        .set_security_opts(SecurityOptions::UsernamePassword(userame, password));
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();

    mqtt_client
        .subscribe(topic.clone(), QoS::AtLeastOnce)
        .unwrap();

    println!(
        "Listening for topic: {} on http://{}:{}",
        topic, broker_ip, broker_port
    );

    for notification in &notifications {
        match notification {
            Notification::Publish(publish) => {
                handle_sensor_data(publish.payload.to_vec(), &channel, &keystore).await
            }
            _ => (),
        }
    }
}
