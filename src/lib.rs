#![no_std]
#![no_main]

#[cfg(feature = "rp2040")]
pub use dht_rp::{DHTSensor};

#[cfg(feature = "rp2040")]
mod dht_rp;

#[derive(Debug, Clone)]
pub enum DHTSensorError {
    Timeout,
    ChecksumError,
    InvalidData,
}

const WAIT_FOR_READINESS_LEVEL_THRESHOLD: u32 = 80;
const LOW_LEVEL_THRESHOLD: u32 = 55;
const HIGH_LEVEL_THRESHOLD: u32 = 75;
