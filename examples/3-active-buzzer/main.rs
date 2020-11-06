#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB4;
use arduino_uno::prelude::*;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // will control the LED with on/off buttons
    let mut buzz_ctrl = PassiveBuzzerCTRL::new(pins);
    buzz_ctrl.start();
}

struct PassiveBuzzerCTRL {
    out: PB4<Output>,
    iterations: u16,
    delays: &'static [u16],
}

impl PassiveBuzzerCTRL {
    fn new(mut pins: arduino_uno::Pins) -> Self {
        PassiveBuzzerCTRL {
            out: pins.d12.into_output(&mut pins.ddr),
            iterations: 1000,
            delays: &[1, 2, 3, 100],
        }
    }
    fn start(&mut self) -> ! {
        let mut curr = 0;
        loop {
            self.buzz(self.delays[curr % self.delays.len()]);
            curr += 1;
        }
    }
    fn buzz(&mut self, delay: u16) {
        for _ in 0..self.iterations {
            self.out.set_high().void_unwrap();
            arduino_uno::delay_ms(delay);
            self.out.set_low().void_unwrap();
            arduino_uno::delay_ms(delay);
        }
    }
}
