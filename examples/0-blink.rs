#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::prelude::*;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output(&mut pins.ddr);

    led.set_high().void_unwrap();

    loop {
        stutter_blink(&mut led, 25);
    }
}

fn stutter_blink(led: &mut PB5<Output>, times: u16) {
    (0..times).map(|i| i * 77).for_each(|i| {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(i);
    });
}
