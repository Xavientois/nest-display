use nest_display::*;
use std::sync::mpsc;
use std::thread;

use anyhow::Result;
use log::{info, trace};

use embedded_hal::blocking::delay::DelayMs;
use rppal::hal::Delay;

// For DHT11 Hygrothermograph
const DHT11_PIN: u8 = 17;
// For LCD Display
const I2C_BUS: u8 = 1;
const I2C_ADDRESS: u16 = 0x3F;

fn main() -> Result<()> {
    env_logger::init();

    info!("Initializing DHT11");
    let mut dht11 = dht11::init(DHT11_PIN)?;

    info!("Initializing LCD Display");
    let display = lcd::init(I2C_BUS, I2C_ADDRESS)?;
    let (display_tx, display_rx) = mpsc::channel();
    thread::spawn(move || run_display(display_rx, display));

    info!("Initializing Nest Client");
    let nest_display_tx = display_tx.clone();
    thread::spawn(move || run_nest(nest_display_tx, Delay::new()));

    loop {
        trace!("Performing measurement");
        let mut delay = Delay::new();
        match dht11.perform_measurement(&mut delay) {
            Ok(measurement) => {
                trace!("Displaying reading");
                let output = format_measurement(&measurement);
                let _ = display_tx.send(DisplayUpdate::First(output));
            }
            Err(e) => info!("Failed to perform measurement: {e:?}"),
        }
        delay.delay_ms(500u16);
    }
}
