use std::sync::mpsc;

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use log::{error, info, trace};
use rppal_dht11::Dht11;

/// Communicate with the DHT11 device over the provided GPIO/BCM pin
pub mod dht11;

/// Communicate with the LCD display over the provided GPIO/BCM pin
pub mod lcd;
pub use crate::lcd::TwoStringPrint;

/// Communicate with remote Nest device
pub mod nest;

/// Display formatting helpers
mod format;

pub type Kill = ();

pub enum DisplayUpdate {
    First(String),
    Second(String),
}

pub fn run_display<Display, Delay>(
    display_rx: mpsc::Receiver<DisplayUpdate>,
    kill_rx: mpsc::Receiver<Kill>,
    mut display: Display,
    mut delay: Delay,
) -> Display
where
    Display: TwoStringPrint,
    Delay: DelayMs<u16>,
{
    use DisplayUpdate::*;

    let mut line1 = String::new();
    let mut line2 = String::new();
    loop {
        // Update display
        for update in display_rx.try_iter() {
            match update {
                First(s) => line1 = s,
                Second(s) => line2 = s,
            }
            display.print_two(&line1, &line2);
        }

        // Check for kill message
        if let Ok(_) = kill_rx.try_recv() {
            break;
        }

        delay.delay_ms(100u16);
    }
    display
}
pub fn run_nest<D: DelayMs<u16>>(tx: mpsc::Sender<DisplayUpdate>, mut delay: D) {
    let nest = nest::Client::new();
    loop {
        trace!("Requesting Nest data");
        let nest_data = nest.get_data();
        if let Ok(data) = nest_data {
            let result = tx.send(DisplayUpdate::Second(format::nest_data(&data)));
            // Kill thread when receiver disconnects
            if result.is_err() {
                break;
            }
        } else {
            error!("{}", nest_data.unwrap_err());
        }

        delay.delay_ms(6001u16);
    }
}

pub fn run_dht11<D>(tx: mpsc::Sender<DisplayUpdate>, mut dht11: Dht11, mut delay: D)
where
    D: DelayMs<u16> + DelayUs<u16>,
{
    loop {
        trace!("Performing measurement");
        match dht11.perform_measurement(&mut delay) {
            Ok(measurement) => {
                trace!("Displaying reading");
                let output = format::measurement(&measurement);
                let result = tx.send(DisplayUpdate::First(output));
                // Kill thread when receiver disconnects
                if result.is_err() {
                    break;
                }
            }
            Err(e) => info!("Failed to perform measurement: {e:?}"),
        }
        delay.delay_ms(500u16);
    }
}
