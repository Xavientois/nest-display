use super::nest;
use rppal_dht11::Measurement;

const DEGREE_SYM: &str = unsafe { std::str::from_utf8_unchecked(b"\xDF") };

// Output Formatting Functions
pub fn measurement(m: &Measurement) -> String {
    const DEGREE_SYM: &str = unsafe { std::str::from_utf8_unchecked(b"\xDF") };
    let (temperature, humidity) = (m.temperature as f64 / 10.0, m.humidity as f64 / 10.0);
    format!("{temperature:.1}{DEGREE_SYM}C | {humidity:.1}%")
}

pub fn nest_data(data: &nest::Data) -> String {
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
