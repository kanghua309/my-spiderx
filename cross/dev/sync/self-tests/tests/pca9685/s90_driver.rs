#![no_main]
#![no_std]
#![allow(dead_code, unused_imports)]

use defmt_rtt as _;
use microbit::hal::{Timer, Twim};
use microbit::pac::{TIMER0, TWIM0};
use panic_probe as _;
// use pwm_pca9685::Pca9685;

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
    use animate::{animate, Move};
    use s90::{Degrees, F64Ext, Servo};
    use s90::i2c_s90::S90;
    use crate::State;


    #[init]
    fn setup() -> State {
        defmt::println!("test init");
        let board = microbit::Board::take().unwrap();
        let i2c = { twim::Twim::new(board.TWIM0, board.i2c_external.into(), FREQUENCY_A::K100) };
        let timer = Timer::new(board.TIMER0);
        State {i2c,timer}
    }
    // #[test]
    // fn set_angle(state: &mut State) {
    //     defmt::println!("set_angle");
    //     // Turn on channel 0 at 0
    //     let mut s90 = S90::new(&mut state.i2c,
    //                            0x80,
    //                            0,
    //                            false).unwrap();
    //     loop {
    //         s90.write(Degrees(0.0));
    //         defmt::println!("curent angle:{}",s90.read().0);
    //         state.timer.delay_ms(1_000_u16 as u32);
    //         for index in 0..1 {
    //             defmt::println!("index:{}",index);
    //             for angle in 0..90 {
    //                 s90.write(Degrees(angle as f64));
    //                 state.timer.delay_ms(1_00_u16 as u32);
    //                 defmt::println!("curent angle:{}",s90.read().0)
    //             }
    //         }
    //     }
    // }
    // #[test]
    // fn set_angle_revert(state: &mut State) {
    //     defmt::println!("set_angle");
    //     // Turn on channel 0 at 0
    //     let mut s90 = S90::new(&mut state.i2c,
    //                            0x80,
    //                            0,
    //                            Degrees(90.0),
    //                            true).unwrap();
    //     loop {
    //         defmt::println!("curent angle:{}",s90.read().0);
    //         state.timer.delay_ms(1_0000_u16 as u32);
    //         for index in 0..1 {
    //             defmt::println!("index:{}",index);
    //             for angle in 0..180 {
    //                 s90.write(Degrees(angle as f64));
    //                 state.timer.delay_ms(1_00_u16 as u32);
    //                 defmt::println!("curent angle:{}",s90.read().0)
    //             }
    //         }
    //     }
    // }

    #[test]
    fn simple_animate(state: &mut State) {
        defmt::println!("simple_animate?");
        // Turn on channel 0 at 0
        let mut hip_front_left = S90::new(& state.i2c,
                                         0x80,
                                         0,
                                         false).unwrap();
        let mut hip_front_right = S90::new(& state.i2c,
                                          0x80,
                                          1,
                                          true).unwrap();
        let mut hip_rear_left = S90::new(& state.i2c,
                                           0x80,
                                           2,
                                           true).unwrap();
        let mut hip_rear_right = S90::new(& state.i2c,
                                         0x80,
                                         3,
                                         false).unwrap();

        let mut knee_front_left = S90::new(& state.i2c,
                                           0x80,
                                           15,
                                           false).unwrap();

        let mut knee_front_right = S90::new(& state.i2c,
                                           0x80,
                                           14,
                                           true).unwrap();
        let mut knee_rear_left = S90::new(& state.i2c,
                                            0x80,
                                            13,
                                            true).unwrap();
        let mut knee_rear_right = S90::new(& state.i2c,
                                          0x80,
                                          12,
                                          false).unwrap();

        //init

        hip_front_left.write(Degrees(45.0));
        hip_front_right.write(Degrees(45.0));
        hip_rear_left.write(Degrees(45.0));
        hip_rear_right.write(Degrees(45.0));

        knee_front_left.write(Degrees(135.0));
        knee_front_right.write(Degrees(135.0));
        knee_rear_left.write(Degrees(135.0));
        knee_rear_right.write(Degrees(135.0));
        // state.timer.delay_ms(1_000_u16 as u32);
        // hip_rear_left.write(Degrees(0.0));
        // state.timer.delay_ms(1_000_u16 as u32);
        // animate(
        //     &mut [
        //         Move::new(&mut knee_rear_right, 180.0.degrees()),
        //         //Move::new(&mut knee_rear_left, 0.0.degrees()),
        //     ],
        //     5000,
        //     &mut state.timer,
        // );
        // loop {
        //     defmt::println!("loop ... ");
        //     animate(
        //         &mut [
        //             Move::new(&mut hip_rear_left, 135.0.degrees()),
        //             Move::new(&mut knee_rear_left, 90.0.degrees()),
        //         ],
        //         5000,
        //         &mut state.timer,
        //     );
        //     state.timer.delay_ms(1_000_u16 as u32);
        //     animate(
        //         &mut [
        //             Move::new(&mut hip_rear_left, 90.0.degrees()),
        //             Move::new(&mut knee_rear_left, 0.0.degrees()),
        //         ],
        //         5000,
        //         &mut state.timer,
        //     );
        //     state.timer.delay_ms(1_000_u16 as u32);
        // }
        let speed = 2000;
        for _ in 0..100 {
            animate(
                &mut [
                    Move::new(&mut knee_front_left, 100.0.degrees()),
                    Move::new(&mut knee_rear_right, 100.0.degrees()),
                ],
                speed,
                &mut state.timer,
            );
            animate(
                &mut [
                    Move::new(&mut hip_front_left, 20.0.degrees()),
                    Move::new(&mut hip_rear_right, 70.0.degrees()),
                ],
                speed,
                &mut state.timer,
            );
            animate(
                &mut [
                    Move::new(&mut knee_front_left, 165.0.degrees()),
                    Move::new(&mut knee_rear_right, 165.0.degrees()),
                ],
                speed,
                &mut state.timer,
            );
            animate(
                &mut [
                    Move::new(&mut hip_front_left, 45.0.degrees()),
                    Move::new(&mut hip_rear_right, 45.0.degrees()),
                ],
                speed,
                &mut state.timer,
            );
            animate(
                &mut [
                    Move::new(&mut knee_front_left, 135.0.degrees()),
                    Move::new(&mut knee_rear_right, 135.0.degrees()),
                ],
                speed,
                &mut state.timer,
            );
            //**********
            animate(
                &mut [
                    Move::new(&mut knee_front_right, 100.0.degrees()),
                    Move::new(&mut knee_rear_left, 100.0.degrees()),
                ],
                speed,
                &mut state.timer,
            );
            animate(
                &mut [
                    Move::new(&mut hip_front_right, 20.0.degrees()),
                    Move::new(&mut hip_rear_left, 70.0.degrees()),
                ],
                speed,
                &mut state.timer,
            );

            animate(
                &mut [
                    Move::new(&mut knee_front_right, 165.0.degrees()),
                    Move::new(&mut knee_rear_left, 165.0.degrees()),
                ],
                speed,
                &mut state.timer,
            );
            animate(
                &mut [
                    Move::new(&mut hip_front_right, 45.0.degrees()),
                    Move::new(&mut hip_rear_left, 45.0.degrees()),
                ],
                speed,
                &mut state.timer,
            );
            animate(
                &mut [
                    Move::new(&mut knee_front_right, 135.0.degrees()),
                    Move::new(&mut knee_rear_left, 135.0.degrees()),
                ],
                speed,
                &mut state.timer,
            );
        }
    }
}
