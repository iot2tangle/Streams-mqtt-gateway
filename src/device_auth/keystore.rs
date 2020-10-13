use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::{Deserialize, Serialize};

use std::fs::File;

static PATH: &str = "src/device_auth/keystore.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Keystore {
    pub api_keys_author: Vec<String>,
}

#[derive(Debug)]
pub struct KeyManager {
    pub keystore: Keystore,
}

impl KeyManager {
    ///
    /// generates a new Keystore object by hashing the plaintext key and stroing it localy
    ///
    pub fn new(new_keys_auth: Vec<String>) -> KeyManager {
        let mut hash_list = vec![];
        for key in new_keys_auth {
            hash_list.push(calculate_hash(key));
        }

        let keystore = Keystore {
            api_keys_author: hash_list.clone(),
        };

        store_keystore(&keystore);

        KeyManager { keystore: keystore }
    }

    ///
    /// recreates the API key struct from local storeg
    ///
    pub fn restore() -> KeyManager {
        let rec: Keystore = serde_json::from_reader(File::open(PATH).unwrap()).unwrap();
        KeyManager { keystore: rec }
    }
}

///
/// stores the current keystore in a local file
///
fn store_keystore(keystore: &Keystore) -> () {
    serde_json::to_writer(&File::create(PATH).unwrap(), keystore).unwrap();
}

///
/// computes a SHA3_256 hash of the provided String
///
pub fn calculate_hash(t: String) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(&t);
    let hex = hasher.result_str();
    hex
}

///
/// Verify that the key provided matches one of the whitelited hashes
///
pub fn authenticate(key: &str, hashes: Vec<String>) -> bool {
    for hash in hashes {
        if calculate_hash(key.to_string()) == hash {
            return true;
        }
    }
    return false;
}
