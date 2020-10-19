use crate::device_auth::keystore::{authenticate, calculate_hash, KeyManager};
use crate::timestamp_in_sec;
use crate::types::sensor_data::SensorData;
use std::sync::{Arc, Mutex};

use gateway_core::gateway::publisher::Channel;

///
/// Handles the reuqest from the sensor by parsing the provieded data into the SensorData Format.
/// It authenticates the device through the "device" attribute, and if successfull published the data to the Tangle
/// through the streams channel
///
pub fn handle_sensor_data(
    data: String,
    channel: &Arc<Mutex<Channel>>,
    keystore: &Arc<Mutex<KeyManager>>,
) -> () {
    let data = data.to_owned();
    //let clean_str = std::str::from_utf8(&data).unwrap().replace("\u{0}", "");
    let json_data: serde_json::Result<SensorData> = serde_json::from_str(&data);
    match json_data {
        Ok(mut sensor_data) => {
            let hash = keystore
                .lock()
                .expect("lock keystore")
                .keystore
                .api_keys_author
                .clone();
            if authenticate(&sensor_data.device, hash.clone()) {
                sensor_data.device.to_string().push_str("_id");
                sensor_data.device = calculate_hash(sensor_data.device);
                sensor_data.timestamp = serde_json::Value::from(timestamp_in_sec());
                println!(
                    "New Message Recieved -- {:?} -- authorized request by device",
                    timestamp_in_sec()
                );
                let mut channel = channel.lock().unwrap();
                match channel.write_signed(&sensor_data) {
                    Ok(msg_id) => println!("{:?}", msg_id),
                    Err(_e) => {
                        println!("Error: Could not send data to Tangle, try switching nodes");
                        ()
                    }
                };
            } else {
                println!(
                    "New Message Recieved -- {:?} -- unauthorized request blocked",
                    timestamp_in_sec()
                )
            }
        }
        Err(_e) => {
            println!(
                "New Message Recieved -- {:?} -- incorrectly formatted Data",
                timestamp_in_sec()
            );
        }
    }
    ()
}
