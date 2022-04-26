use nest_display::*;
use std::sync::mpsc;
use std::{panic, thread};

use anyhow::Result;
use log::info;

use rppal::hal::Delay;

// For DHT11 Hygrothermograph
const DHT11_PIN: u8 = 17;
// For LCD Display
const I2C_BUS: u8 = 1;
const I2C_ADDRESS: u16 = 0x3F;

fn main() -> Result<()> {
    env_logger::init();

    info!("Initializing Termination Handling");
    let (kill_tx, kill_rx) = mpsc::channel();
    ctrlc::set_handler(move || kill_tx.send(()).expect("Could not send kill message"))?;

    info!("Initializing LCD Display");
    let display = lcd::init(I2C_BUS, I2C_ADDRESS)?;
    let (display_tx, display_rx) = mpsc::channel();
    let display_thread =
        thread::spawn(move || run_display(display_rx, kill_rx, display, Delay::new()));

    info!("Initializing DHT11");
    let dht11 = dht11::init(DHT11_PIN)?;
    let dht11_display_tx = display_tx.clone();
    thread::spawn(move || run_dht11(dht11_display_tx, dht11, Delay::new()));

    info!("Initializing Nest Client");
    let nest_display_tx = display_tx.clone();
    thread::spawn(move || run_nest(nest_display_tx, Delay::new()));

    info!("Shutting down");
    match display_thread.join() {
        Err(e) => panic::resume_unwind(e),
        Ok(display) => {
            info!("Turning off LCD Display");
            lcd::turn_off(display);
        }
    }

    Ok(())
}
