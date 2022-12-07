#![no_main]
#![no_std]
#![allow(dead_code, unused_imports)]

use defmt_rtt as _;
use microbit::hal::{Timer, Twim};
use microbit::pac::{TIMER0, TWIM0, TWIM1};
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
    use embedded_hal::blocking::i2c;
    use microbit::{
        hal::{
            gpio::{p0, p1, Disconnected, Level, OpenDrainConfig::Disconnect0HighDrive1},
            twim, twis, uarte,
        },
        pac,
    };
    use microbit::board::I2CExternalPins;
    use microbit::hal::timer::Timer;
    use microbit::hal::Twim;
    use microbit::pac::TWIM0;
    use pwm_pca9685::{Address, Channel, Pca9685};
    use microbit::hal::prelude::*;
    use microbit::pac::twim0::frequency::FREQUENCY_A;
    use crate::State;


    #[init]
    fn setup() -> State {
        defmt::println!("test init");
        // let p = microbit::Peripherals::take().unwrap();
        // //let board = microbit::Board::take().unwrap();
        //
        // let p0parts = p0::Parts::new(p.P0);
        // let p1parts = p1::Parts::new(p.P1);
        // let scl = p0parts.p0_26.into_floating_input();
        // let sda = p1parts.p1_00.into_floating_input();
        //
        //
        // let pins =  twim::Pins {
        //     scl: scl.degrade(),
        //     sda: sda.degrade(),
        // };

        let board = microbit::Board::take().unwrap();
        let i2c = { twim::Twim::new(board.TWIM0, board.i2c_external.into(), FREQUENCY_A::K100) };
        // let mut pwm = Pca9685::new(i2c, 0x80).unwrap();
        // pwm.enable().unwrap();
        // pwm.set_prescale(100).unwrap();

        let timer = Timer::new(board.TIMER0);
        State {i2c,timer}
    }

    #[test]
    fn simple_set_duty(state: &mut State) {
        defmt::println!("simple_set_duty");

        // let mut acc = [0,150];
        // state.i2c.write(0x80,&acc);
        // state.timer.delay_ms(10000u32);
        let mut acc2 = [1,150];

        state.i2c.write(0x80,&[0,150]);
        loop{
            continue
        }
        // state.timer.delay_ms(10000u32);
        // state.i2c.write(0x80,&[0,0]);
        // state.timer.delay_ms(10000u32);
        // // Turn on channel 0 at 0
        // //state.pca.set_channel_on(Channel::C0, 10).unwrap();
        // let servo_min = 132; // minimum pulse length (out of 4096)
        // let servo_max = 608; // maximum pulse length (out of 4096)
        // let mut current = servo_min;
        //
        // let mut factor: i16 = 1;
        // loop {
        //     // let mut on = [0; 16];
        //     // let mut off = [1000; 16];
        //     //state.pca.set_all_on_off(&on, &off).unwrap();
        //
        //     //current = 3500;
        //     defmt::println!("loop:{}",current);
        //     state.pca.set_channel_on_off(Channel::All,0,current).unwrap();
        //
        //     //state.pca.set_channel_on_off(Channel::All,0, current).unwrap();
        //     // state.pca.set_channel_off(Channel::C1, servo_min + (servo_max - current))
        //     //    .unwrap();
        //     if current == servo_max {
        //         factor = -4;
        //     } else if current == servo_min {
        //         factor = 4;
        //     }
        //     current = (current as i16 + factor) as u16;
        //     state.timer.delay_ms(1000u32);
        // }
    }
}
