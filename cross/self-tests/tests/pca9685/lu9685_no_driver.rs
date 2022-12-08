#![no_main]
#![no_std]
#![allow(dead_code, unused_imports)]

use defmt_rtt as _;
use microbit::hal::{Timer, Twim};
use microbit::pac::{TIMER0, TWIM0};
use panic_probe as _;
use pwm_pca9685::Pca9685;

struct State {
    i2c:Twim<TWIM0>,
    timer:Timer<TIMER0>,
}

#[defmt_test::tests]
mod tests {
    use core::time::Duration;
    use defmt::{assert_eq, unwrap};
    use embedded_hal::Pwm;
    use embedded_hal::digital::v2::OutputPin;
    use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
    use microbit::{
        hal::{
            clocks::Clocks,
            gpio,
            pwm,
            rtc::{Rtc, RtcInterrupt},
            time::Hertz,
            twim,
        },
        pac::{self, interrupt},
        pac::twim0::frequency::FREQUENCY_A,
        board,
    };
    use microbit::hal::timer::Timer;
    use microbit::hal::Twim;
    use microbit::pac::TWIM0;
    use pwm_pca9685::{Address, Channel, Pca9685};
    use crate::State;


    #[init]
    fn setup() -> State {
        defmt::println!("test init");
        let board = microbit::Board::take().unwrap();
        let i2c = { twim::Twim::new(board.TWIM0, board.i2c_external.into(), FREQUENCY_A::K100) };
        //let address = Address::default();//0b100_0000
        // let mut pwm = Pca9685::new(i2c, 0x80).unwrap();
        // pwm.enable().unwrap();
        // pwm.set_prescale(100).unwrap();

        let timer = Timer::new(board.TIMER0);
        State {i2c,timer}
    }

    #[test]
    fn simple_set_duty(state: &mut State) {
        defmt::println!("simple_set_duty");
        // Turn on channel 0 at 0
        loop {
            for index in (0..16) {
                defmt::println!("index:{}",index);
                for angle in (0..250) {
                    let mut acc = [index, angle];
                    state.timer.delay_ms(1_0_u16 as u32);
                }
            }
        }
    }
}
