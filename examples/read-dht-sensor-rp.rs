#![no_std]
#![no_main]


use embassy_executor::Spawner;
use embassy_rp::gpio::{AnyPin, Flex};
use embassy_time::{Duration, Timer};
use embassy_dht_sensor::dht_rp::DHTSensor;
use {defmt::info, defmt_rtt as _, panic_probe as _};
use embassy_dht_sensor::DHTSensorError;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_rp::init(Default::default());

    let pin = Flex::new(AnyPin::from(p.PIN_0));

    let mut dht_sensor = DHTSensor::new(pin);
    loop {
        match dht_sensor.read() {
            Ok(data) => {
                info!("temperature: {:?}, humidity: {:?}", data.temperature, data.humidity);
            }
            Err(e) => {
                match e {
                    DHTSensorError::Timeout => {
                        info!("Timeout");
                    }
                    DHTSensorError::ChecksumError => {
                        info!("CRC error");
                    }
                    DHTSensorError::InvalidData => {
                        info!("Invalid data");
                    }
                }
            }
        }
        Timer::after(Duration::from_secs(1)).await;
    }
}
