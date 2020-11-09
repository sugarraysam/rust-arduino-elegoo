#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;
use arduino_uno::atmega328p::TC1;
use arduino_uno::hal::port;
use arduino_uno::prelude::*;
use arduino_uno::pwm::Prescaler;
use elegoo::tone::pitches::*;

// 16 MhHz == CPU frequency for Arduino
const F_CPU: u32 = 16000000;
static mut PIN: Option<port::portb::PB0<port::mode::Output>> = None;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut out_pin = pins.d8.into_output(&mut pins.ddr);
    out_pin.set_low().void_unwrap();
    unsafe {
        PIN = Some(out_pin);
        avr_device::interrupt::enable();
    }
    // let melody: &[u16] = &[
    //     NOTE_C5, NOTE_D5, NOTE_E5, NOTE_F5, NOTE_G5, NOTE_A5, NOTE_B5, NOTE_C6, NOTE_DS8, NOTE_B0,
    // ];
    let melody = &[NOTE_A5, NOTE_E5, NOTE_G5];
    let buzz = PassiveBuzzerCTRL::new(dp.TC1, melody);
    buzz.start()

    // TODO - log output to screen as program is running (e.g: ocr value, current note (hz) played)
}

struct PassiveBuzzerCTRL<'a> {
    tc1: TC1,
    melody: &'a [u16],
}

impl<'a> PassiveBuzzerCTRL<'a> {
    fn new(tc1: TC1, melody: &'a [u16]) -> Self {
        let buzz = PassiveBuzzerCTRL { tc1, melody };
        buzz.init();
        buzz
    }
    fn init(&self) {
        // reset compA value to 0x00
        self.tc1.tccr1a.write(|w| unsafe { w.bits(0x00) });

        // no prescaler - bitWrite(TCCR1B, CS10, 1);
        // CTC mode (tops at OCR1A) - bitWrite(TCCR1B, WGM12, 1);
        self.tc1.tccr1b.write(|w| {
            w.cs1().direct();
            w.wgm1().bits(0b100)
        });
    }
    fn start(&self) -> ! {
        loop {
            for &note in self.melody {
                self.play(note as u32);
                arduino_uno::delay_ms(2000);
            }
            self.stop();
            arduino_uno::delay_ms(3000);
        }
    }

    fn play(&self, freq: u32) {
        let mut ocr = F_CPU / freq / 2 - 1 as u32;
        let mut p = Prescaler::Direct;
        // max counter value 16bit == 0xffff
        if ocr > 0xffff {
            ocr = F_CPU / freq / 2 / 64 - 1;
            p = Prescaler::Prescale64;
        }
        // set prescaler - TCCR1B = (TCCR1B & 0b11111000) | prescalarbits;
        self.tc1.tccr1b.write(|w| match p {
            Prescaler::Direct => w.cs1().direct(),
            Prescaler::Prescale64 => w.cs1().prescale_64(),
            _ => w,
        });
        // set compA value - OCR1A = ocr;
        self.tc1.ocr1a.write(|w| unsafe { w.bits(ocr as u16) });

        // enable compA match interrupt - bitWrite(TIMSK1, OCIE1A, 1);
        self.tc1.timsk1.write(|w| w.ocie1a().set_bit())
    }

    fn _is_playing(&self) -> bool {
        self.tc1.timsk1.read().ocie1a().bit_is_set()
    }

    // disable compA match interrupt
    fn stop(&self) {
        self.tc1.timsk1.write(|w| w.ocie1a().clear_bit())
    }
}

#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER1_COMPA() {
    PIN.as_mut().unwrap().toggle().void_unwrap();
}
