#![no_std]
#![no_main]

use panic_halt as _;

const MORSE_UNIT = 500;

enum MorseChar {
  Dah,
  Dit
}

fn to_morse(character: char) {
  use MoorseChar::{Dit, Dah};

  let _ = match character {
    'a' => [Dit, Dah],
    'b' => [Dah, Dit, Dit, Dit],
    'c' => [Dah, Dit, Dah, Dit],
    'd' => [Dah, Dit, Dit],
    'e' => [Dit],
    'f' => [Dit, Dit, Dah, Dit],
    'g' => [Dah, Dah, Dit],
    'h' => [Dit, Dit, Dit, Dit],
    'i' => [Dit, Dit],
    'j' => [Dit, Dah, Dah, Dah],
    'k' => [Dah, Dit, Dah],
    'l' => [Dit, Dah, Dit, Dit],
    'm' => [Dah, Dah],
    'n' => [Dah, Dit],
    'o' => [Dah, Dah, Dah],
    'p' => [Dit, Dah, Dah, Dit],
    'q' => [Dah, Dah, Dit, Dah],
    'r' => [Dit, Dah, Dit],
    's' => [Dit, Dit, Dit],
    't' => [Dah],
    'u' => [Dit, Dit, Dah],
    'v' => [Dit, Dit, Dit, Dah],
    'w' => [Dit, Dah, Dah],
    'x' => [Dah, Dit, Dit, Dah],
    'y' => [Dah, Dit, Dah, Dah],
    'z' => [Dah, Dah, Dit, Dit],
    _ => unimplemented!("No morse conversion for char")
  };
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();
    led.set_low();

    loop {
        led.toggle();
        arduino_hal::delay_ms(500);
        led.toggle();
        arduino_hal::delay_ms(500);
        led.toggle();
        arduino_hal::delay_ms(500);
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}

