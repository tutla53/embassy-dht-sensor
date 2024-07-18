use cortex_m::interrupt::free;
use embassy_rp::gpio::{AnyPin, Flex, Level, Pull};
use embassy_rp::gpio::Level::{High, Low};
use embassy_time::{block_for, Duration};
use crate::{DHTSensorError, HIGH_LEVEL_THRESHOLD, LOW_LEVEL_THRESHOLD, WAIT_FOR_READINESS_LEVEL_THRESHOLD};
use crate::DHTSensorError::InvalidData;

#[derive(Clone)]
pub struct DTHResponse {
    pub humidity: f32,
    pub temperature: f32,
}

pub struct DHTSensor<'a> {
    pin: Flex<'a, AnyPin>,
    last_response: Option<DTHResponse>,
}

impl<'a> DHTSensor<'a> {
    pub fn new(pin: Flex<'a, AnyPin>) -> Self {
        DHTSensor {
            pin,
            last_response: None,
        }
    }

    pub fn read(&mut self) -> Result<DTHResponse, DHTSensorError> {
        match self.read_raw_data() {
            Ok(data) => {
                let humidity = ((data[0] << 8) | data[1]) as f32 / 10.0;
                let mut temperature = (((data[2] & 0x7f) << 8) | data[3]) as f32 / 10.0;
                if data[2] & 0x80 != 0 {
                    temperature = -temperature;
                }
                if humidity <= 100.0 {
                    let response = DTHResponse {
                        humidity,
                        temperature,
                    };
                    self.last_response = Some(response.clone());
                    Ok(response)
                }
                else {
                    if let Some(response) = &self.last_response {
                        Ok(response.clone())
                    } else {
                        Err(InvalidData)
                    }
                }
            }
            Err(e) => {
                if let Some(response) = &self.last_response {
                    Ok(response.clone())
                } else {
                    Err(e)
                }
            }
        }
    }

    fn read_raw_data(&mut self) -> Result<[u16; 5], DHTSensorError> {
        let mut data: [u16; 5] = [0; 5];
        let mut all_bits_cycles: [u32; 80] = [0; 80];

        free(|_| {

            // Wake up the sensor
            self.pin.set_as_output();
            self.pin.set_low();
            block_for(Duration::from_micros(1100u64));

            // Ask for data
            self.pin.set_high();
            block_for(Duration::from_micros(25u64));

            self.pin.set_as_input();
            self.pin.set_pull(Pull::Up);
            block_for(Duration::from_micros(55u64));

            // Wait for DHT to signal data is ready (~80us low followed by ~80us high)
            _ = self.wait_while_level(Low, WAIT_FOR_READINESS_LEVEL_THRESHOLD);
            _ = self.wait_while_level(High, WAIT_FOR_READINESS_LEVEL_THRESHOLD);

            for bit in (0..80).step_by(2) {
                if let Ok(bit_cycles) = self.wait_while_level(Low, LOW_LEVEL_THRESHOLD) {
                    all_bits_cycles[bit] = bit_cycles;
                    if let Ok(bit_cycles) = self.wait_while_level(High, HIGH_LEVEL_THRESHOLD) {
                        all_bits_cycles[bit + 1] = bit_cycles;
                    }
                }
            }

            self.pin.set_as_output();
            self.pin.set_high();
            block_for(Duration::from_micros(1100u64));
        });

        for i in 0..40 {
            let low_cycles = all_bits_cycles[2 * i];
            let high_cycles = all_bits_cycles[2 * i + 1];
            if low_cycles < LOW_LEVEL_THRESHOLD || high_cycles < HIGH_LEVEL_THRESHOLD {
                data[i / 8] <<= 1;
                if high_cycles > low_cycles {
                    data[i / 8] |= 1;
                }
            } else {
                return Err(DHTSensorError::Timeout);
            }
        }
        let sum = data[0] + data[1] + data[2] + data[3];
        if data[4] == sum & 0x00FF {
            Ok(data)
        } else {
            Err(DHTSensorError::ChecksumError)
        }
    }

    // Wait for the pin to change from the specified level, or until the timeout is reached.
    fn wait_while_level(&mut self, level: Level, timeout_us: u32) -> Result<u32, DHTSensorError> {
        let mut elapsed_us = 0u32;
        while self.pin.get_level() == level && elapsed_us < timeout_us {
            block_for(Duration::from_micros(1u64));
            elapsed_us += 1;
        }
        if elapsed_us >= timeout_us {
            Err(DHTSensorError::Timeout)
        } else {
            Ok(elapsed_us)
        }
    }
}


