use anyhow::Result;
use lcd_pcf8574::Pcf8574;

/// Communicate with the LCD display over the given bus and address
/// The bus should be the number of the i2c bus (ie. `/dev/i2c-*`)
pub fn init(bus: u8, address: u16) -> Result<lcd::Display<Pcf8574>> {
    let mut ic2_module = Pcf8574::new(bus, address)?;
    ic2_module.backlight(true);
    let mut display = lcd::Display::new(ic2_module);
    display.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    display.display(
        lcd::DisplayMode::DisplayOn,
        lcd::DisplayCursor::CursorOff,
        lcd::DisplayBlink::BlinkOff,
    );
    Ok(display)
}

pub trait TwoStringPrint {
    fn print_two(&mut self, s1: &str, s2: &str);
}
impl<T: lcd::Hardware + lcd::Delay> TwoStringPrint for lcd::Display<T> {
    fn print_two(&mut self, s1: &str, s2: &str) {
        self.clear();
        self.home();
        self.print(&s1);
        self.position(0, 1);
        self.print(&s2);
    }
}
