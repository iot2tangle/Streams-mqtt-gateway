//!
//! Channel Library
//!
#![deny()]
//#![cfg_attr(not(debug_assertions), deny(warnings))]
extern crate gateway_core;
pub mod api;
pub mod security;
pub mod types;

use crate::security::keystore::calculate_hash;
fn authenticate(key: &str, hash: String) -> bool {
    calculate_hash(key.to_string()) == hash
}

use std::time::{SystemTime, UNIX_EPOCH};
fn timestamp_in_sec() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
