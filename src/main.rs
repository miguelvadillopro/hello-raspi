extern crate rppal;

use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;

// The GPIO module uses BCM pin numbering. BCM GPIO 18 is tied to physical pin 12.
const GPIO_LED: u8 = 2;

fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let mut gpio = Gpio::new().unwrap();
    gpio.set_mode(GPIO_LED, Mode::Output);

    // Blink an LED attached to the pin on and off
    loop {
    	gpio.write(GPIO_LED, Level::High);
    	thread::sleep(Duration::from_millis(500));
    	gpio.write(GPIO_LED, Level::Low);
    	thread::sleep(Duration::from_millis(500));
    }
}
