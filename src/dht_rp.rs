use embassy_time::{block_for, Duration};
use crate::DHTSensorError;

fn wait_while(timeout_us: u32, condition: impl Fn() -> bool) -> Result<u32, DHTSensorError> {
    let mut elapsed_us = 0u32;
    while condition() && elapsed_us < timeout_us {
        block_for(Duration::from_micros(1u64));
        elapsed_us += 1;
    }
    if elapsed_us >= timeout_us {
        Err(DHTSensorError::Timeout)
    }
    else {
        Ok(elapsed_us)
    }
}
