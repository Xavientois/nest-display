use anyhow::Result;
use rppal::gpio::{Gpio, Mode};
use rppal_dht11::Dht11;

pub fn init(pin: u8) -> Result<Dht11> {
    let pin = Gpio::new()?.get(pin)?.into_io(Mode::Output);
    Ok(Dht11::new(pin))
}
