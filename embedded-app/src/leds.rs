use linux_embedded_hal::I2cdev;
use mcp23017::MCP23017;

pub const LEDS_IC2_ADDRESS: u8 = 0x20;

pub struct LEDS {
    expander: MCP23017<I2cdev>,
}

impl LEDS {
    pub fn new() -> Self {
        let device = I2cdev::new("/dev/i2c-1").expect("Could not get ic2 device");
        let mut expander =
            MCP23017::new(device, LEDS_IC2_ADDRESS).expect("Could not get led expander");
        expander.all_pin_mode(mcp23017::PinMode::OUTPUT).unwrap(); // set all pins to outputs
        expander.write_gpioab(0).unwrap(); // set all pins to 0

        LEDS { expander }
    }

    pub fn light_leds(&mut self, leds: [i32; 16]) {
        println!("light em up {:?}", leds);
        let as_number = leds
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &bit)| acc | (bit as u16) << i);
        self.expander.write_gpioab(as_number).unwrap();
    }

    pub fn light_led(&mut self, led: i32) {
        println!("lighting {}", led);
        self.expander.write_gpioab(led as u16).unwrap()
    }
}
