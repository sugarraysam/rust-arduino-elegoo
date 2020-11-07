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

    let mut buzz_ctrl = PassiveBuzzerCTRL::new(pins);
    buzz_ctrl.start();
}

struct PassiveBuzzerCTRL {
    out: PB4<Output>,
    iterations: &'static [u32],
    delays: &'static [u16],
}

impl PassiveBuzzerCTRL {
    fn new(mut pins: arduino_uno::Pins) -> Self {
        PassiveBuzzerCTRL {
            out: pins.d12.into_output(&mut pins.ddr),
            iterations: &[10, 100],
            delays: &[0, 1, 2, 50, 100, 250],
        }
    }
    fn start(&mut self) -> ! {
        // using closure iterators because vec! macro and most rust
        // iterator constructs require std lib, which we dont have access
        // same thing for the external rng crate
        let mut next_n = closure_iterator(self.iterations);
        let mut next_delay = closure_iterator(self.delays);
        loop {
            self.buzz(next_n(), next_delay());
        }
    }
    fn buzz(&mut self, n: u32, delay: u16) {
        for _ in 0..n {
            self.out.set_high().void_unwrap();
            arduino_uno::delay_ms(delay);
            self.out.set_low().void_unwrap();
            arduino_uno::delay_ms(delay);
        }
    }
}

fn closure_iterator<'a, T: Copy>(values: &'a [T]) -> impl FnMut() -> T + 'a {
    let mut next = 0;
    move || -> T {
        let choice = values[next % values.len()];
        next += 1;
        choice
    }
}
