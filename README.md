# Embassy DHT Sensor Library

This Rust library provides an interface for interacting with DHT1X and DHT2X temperature and humidity sensors using the Embassy framework.

Adafruit DHT sensor library is used as a reference for this library.
https://github.com/adafruit/DHT-sensor-library

## Note
This library should be used in **release** mode. The measurements made in the **debug** mode are not accurate enough.

## Supported Devices
Currently only the Raspberry Pi Pico board supported.

## Getting Started

### Installation

Add `embassy-dht-sensor` to your `Cargo.toml`:

```toml
[dependencies]
embassy-dht-sensor = "0.1.0"
```

## Usage
Initialize your Raspberry Pi Pico board with Embassy.
Create an instance of DHTSensor with the GPIO pin connected to your DHT sensor.
Use the read method to get temperature and humidity readings.

### Example:

```rust
use embassy_executor::Spawner;
use embassy_rp::gpio::{AnyPin, Flex};
use embassy_time::{Duration, Timer};
use embassy_dht_sensor::DHTSensor;
use defmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
let p = embassy_rp::init(Default::default());
let pin = Flex::new(AnyPin::from(p.PIN_0));
let mut dht_sensor = DHTSensor::new(pin);

    loop {
        match dht_sensor.read() {
            Ok(data) => {
                info!("Temperature: {:?}, Humidity: {:?}", data.temperature, data.humidity);
            },
            Err(e) => {
                info!("Error reading from DHT sensor: {:?}", e);
            }
        }
        Timer::after(Duration::from_secs(1)).await;
    }
}
'''