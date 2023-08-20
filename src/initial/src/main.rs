#![no_std]
#![no_main]

use arduino_hal::hal::port::PB5;
use arduino_hal::port::{mode::Output, Pin};
use core::time::Duration;
use morsec::{Morsec, MorsecTransmitter};
use panic_halt as _;

struct ArduinoMorse<'a> {
    led: &'a mut Pin<Output, PB5>,
}

impl ArduinoMorse<'_> {
    fn new<'a>(led: &'_ mut Pin<Output, PB5>) -> ArduinoMorse<'_> {
        ArduinoMorse { led }
    }
}

impl MorsecTransmitter for ArduinoMorse<'_> {
    fn sleep(&mut self, duration: Duration) {
        arduino_hal::delay_ms(duration.as_millis() as u16);
    }

    fn toggle(&mut self) {
        self.led.toggle();
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();
    led.set_high();

    loop {
        let hello = ArduinoMorse::new(&mut led);
        let sender = Morsec::new("hello world", hello);
        sender.transmit();
    }
}
