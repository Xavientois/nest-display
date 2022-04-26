use std::sync::mpsc;

use embedded_hal::blocking::delay::DelayMs;
use log::error;
use rppal_dht11::Measurement;

/// Communicate with the DHT11 device over the provided GPIO/BCM pin
pub mod dht11;

/// Communicate with the LCD display over the provided GPIO/BCM pin
pub mod lcd;
pub use crate::lcd::TwoStringPrint;

/// Communicate with remote Nest device
pub mod nest;

const DEGREE_SYM: &str = unsafe { std::str::from_utf8_unchecked(b"\xDF") };

// Thread functions
pub enum DisplayUpdate {
    First(String),
    Second(String),
}

pub fn run_display<D>(rx: mpsc::Receiver<DisplayUpdate>, mut display: D)
where
    D: TwoStringPrint,
{
    use DisplayUpdate::*;

    let mut line1 = String::new();
    let mut line2 = String::new();
    for update in rx.iter() {
        match update {
            First(s) => line1 = s,
            Second(s) => line2 = s,
        }
        display.print_two(&line1, &line2);
    }
}
pub fn run_nest<D: DelayMs<u16>>(tx: mpsc::Sender<DisplayUpdate>, mut delay: D) {
    loop {
        let nest = nest::Client::new();
        let nest_data = nest.get_data();
        if let Ok(data) = nest_data {
            let _ = tx.send(DisplayUpdate::Second(format_nest_data(&data)));
        } else {
            error!("{}", nest_data.unwrap_err());
        }

        delay.delay_ms(6001u16);
    }
}

// Output Formatting Functions
pub fn format_measurement(m: &Measurement) -> String {
    const DEGREE_SYM: &str = unsafe { std::str::from_utf8_unchecked(b"\xDF") };
    let (temperature, humidity) = (m.temperature as f64 / 10.0, m.humidity as f64 / 10.0);
    format!("{temperature:.1}{DEGREE_SYM}C | {humidity:.1}%")
}

fn format_nest_data(data: &nest::Data) -> String {
    use nest::Data::*;
    match data {
        HeatCool {
            heat_point,
            temperature,
            cool_point,
        } => {
            format!("{heat_point:.1}  {temperature:.1}  {cool_point:.1}")
        }
        Heat {
            heat_point,
            temperature,
        } => {
            format!("H:{heat_point:.1}{DEGREE_SYM}C T:{temperature:.1}{DEGREE_SYM}C")
        }
        Cool {
            temperature,
            cool_point,
        } => {
            format!("C:{cool_point:.1}{DEGREE_SYM}C T:{temperature:.1}{DEGREE_SYM}C")
        }
        Off { temperature } => {
            format!("Temp: {temperature:.1}{DEGREE_SYM}C")
        }
    }
}
