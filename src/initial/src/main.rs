#![no_std]
#![no_main]

use panic_halt as _;

const MORSE_UNIT = 500;

enum MorseChar {
  Dash,
  Dot
}

fn to_morse(character: char) {
  use MoorseChar::{Dot, Dash};

  let _ = match character {
    'a' => [Dot, Dash],
    'b' => [Dash, Dot, Dot, Dot],
    'c' => [Dash, Dot, Dash, Dot],
    'd' => [Dash, Dot, Dot],
    'e' => [Dot],
    'f' => [Dot, Dot, Dash, Dot],
    'g' => [Dash, Dash, Dot],
    'h' => [Dot, Dot, Dot, Dot],
    'i' => [Dot, Dot],
    'j' => [Dot, Dash, Dash, Dash],
    'k' => [Dash, Dot, Dash],
    'l' => [Dot, Dash, Dot, Dot],
    'm' => [Dash, Dash],
    'n' => [Dash, Dot],
    'o' => [Dash, Dash, Dash],
    'p' => [Dot, Dash, Dash, Dot],
    'q' => [Dash, Dash, Dot, Dash],
    'r' => [Dot, Dash, Dot],
    's' => [Dot, Dot, Dot],
    't' => [Dash],
    'u' => [Dot, Dot, Dash],
    'v' => [Dot, Dot, Dot, Dash],
    'w' => [Dot, Dash, Dash],
    'x' => [Dash, Dot, Dot, Dash],
    'y' => [Dash, Dot, Dash, Dash],
    'z' => [Dash, Dash, Dot, Dot],
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

