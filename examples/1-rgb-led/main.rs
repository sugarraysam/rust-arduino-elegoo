#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_uno::hal::port::mode::Pwm;
use arduino_uno::hal::port::portd::{PD3, PD5, PD6};
use arduino_uno::prelude::*;
use arduino_uno::pwm::*;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // ATmega328P has three timers, Timer0, Timer1, Timer2, controlling 6 PWM outputs
    // excellent PWM tutorial here: http://www.righto.com/2009/07/secrets-of-arduino-pwm.html
    // - Default timer mode is pwm_fast() for timer2 and timer0
    // - Prescale64 => 977 Hz frequency
    // - timer0 for PD5 & PD6, timer2 for PD3
    let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    // instantiate pins as PWM
    let r = pins.d6.into_output(&mut pins.ddr).into_pwm(&mut timer0);
    let g = pins.d5.into_output(&mut pins.ddr).into_pwm(&mut timer0);
    let b = pins.d3.into_output(&mut pins.ddr).into_pwm(&mut timer2);

    let delay = 200;
    let mut rgb = RGB::new(r, g, b);

    loop {
        rgb.fade_to_red();
        arduino_uno::delay_ms(delay);
        rgb.fade_to_green();
        arduino_uno::delay_ms(delay);
        rgb.fade_to_blue();
        arduino_uno::delay_ms(delay);
        rgb.fade_to_white();
        arduino_uno::delay_ms(delay);
        rgb.fade_to_black();
        arduino_uno::delay_ms(delay);
    }
}

struct RGB {
    r: PD6<Pwm<Timer0Pwm>>,
    g: PD5<Pwm<Timer0Pwm>>,
    b: PD3<Pwm<Timer2Pwm>>,
}

impl RGB {
    // struct to control RGB LED, initialize with red on
    fn new(
        mut r: PD6<Pwm<Timer0Pwm>>,
        mut g: PD5<Pwm<Timer0Pwm>>,
        mut b: PD3<Pwm<Timer2Pwm>>,
    ) -> Self {
        r.set_duty(255);
        r.enable();
        g.set_duty(0);
        g.enable();
        b.set_duty(0);
        b.enable();
        RGB { r, g, b }
    }

    // generic fade function to control target output of RGB led
    fn fade(&mut self, r_target: u8, g_target: u8, b_target: u8) {
        let delay = 10;
        let mut r_val = self.r.get_duty();
        let mut g_val = self.g.get_duty();
        let mut b_val = self.b.get_duty();

        // closure to calculate next val
        let next_val = |cur: u8, target: u8| -> u8 {
            if cur < target {
                cur + 1
            } else {
                cur - 1
            }
        };

        loop {
            let mut n_modified = 0;
            if r_val != r_target {
                r_val = next_val(r_val, r_target);
                self.r.set_duty(r_val);
                n_modified += 1;
            }
            if g_val != g_target {
                g_val = next_val(g_val, g_target);
                self.g.set_duty(g_val);
                n_modified += 1;
            }
            if b_val != b_target {
                b_val = next_val(b_val, b_target);
                self.b.set_duty(b_val);
                n_modified += 1;
            }
            // exit
            if n_modified == 0 {
                break;
            }
            arduino_uno::delay_ms(delay);
        }
    }

    fn fade_to_green(&mut self) {
        self.fade(0, 255, 0);
    }

    fn fade_to_red(&mut self) {
        self.fade(255, 0, 0);
    }

    fn fade_to_blue(&mut self) {
        self.fade(0, 0, 255);
    }

    fn fade_to_black(&mut self) {
        self.fade(0, 0, 0);
    }

    fn fade_to_white(&mut self) {
        self.fade(255, 255, 255);
    }
}
