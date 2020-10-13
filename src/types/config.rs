use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub whitelisted_device_ids: Vec<String>,
    pub username: String,
    pub password: String,
    pub broker_ip: String,
    pub broker_port: u16,
    pub topic: String,
    pub node: String,
    pub mwm: u8,
    pub local_pow: bool,
}
