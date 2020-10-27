#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_uno::hal::port::mode::{Input, Output, PullUp};
use arduino_uno::hal::port::portb::{PB0, PB1};
use arduino_uno::hal::port::portd::PD5;
use arduino_uno::prelude::*;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // will control the LED with on/off buttons
    let mut led_ctrl = LEDController::new(pins);
    led_ctrl.start();
}

struct LEDController {
    on_button: PB1<Input<PullUp>>,
    off_button: PB0<Input<PullUp>>,
    led: PD5<Output>,
}

impl LEDController {
    fn new(mut pins: arduino_uno::Pins) -> Self {
        // initiaize LED on
        let mut led = pins.d5.into_output(&mut pins.ddr);
        led.set_high().void_unwrap();

        // right button => on, left button => off
        LEDController {
            on_button: pins.d9.into_pull_up_input(&mut pins.ddr),
            off_button: pins.d8.into_pull_up_input(&mut pins.ddr),
            led,
        }
    }

    // we can use return the never type ("!") inside our struct for cleaner code :D
    fn start(&mut self) -> ! {
        loop {
            if self.on_button.is_low().void_unwrap() {
                self.led.set_high().void_unwrap();
            }
            if self.off_button.is_low().void_unwrap() {
                self.led.set_low().void_unwrap();
            }
        }
    }
}
