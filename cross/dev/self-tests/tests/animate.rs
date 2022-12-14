#![no_main]
#![no_std]
#![allow(dead_code, unused_imports)]

use defmt_rtt as _;
use panic_probe as _;
use animate::*;

//use embedded_hal::Pwm;
//use embedded_hal::digital::v2::OutputPin;
// use microbit::{
//     hal::{
//         clocks::Clocks,
//         gpio,
//         prelude::OutputPin,
//         pwm,
//         rtc::{Rtc, RtcInterrupt},
//         time::Hertz,
//     },
//     pac::{self, interrupt},
//     board,
// };
use microbit::hal::pwm::Pwm;
use microbit::hal::timer::Timer;
use microbit::pac::TIMER0;
use s90::pwm_s90::S90;

struct State {
    //pwm: Pwm<microbit::hal::pac::PWM0>,
    s90:S90<Pwm<microbit::hal::pac::PWM0>,microbit::hal::pwm::Channel>,
    s90_revert:S90<Pwm<microbit::hal::pac::PWM1>,microbit::hal::pwm::Channel>,
    timer:Timer<TIMER0>,
}

#[defmt_test::tests]
mod tests {
    use core::time::Duration;
    use defmt::{assert_eq, unwrap};
    use embedded_hal::digital::v2::OutputPin;
    use embedded_hal::prelude::{_embedded_hal_blocking_delay_DelayMs, _embedded_hal_blocking_spi_Write};
    use embedded_hal::Pwm;
    use super::State;
    use animate::{animate, Move};
    use s90::{F64Ext, Servo};
    use microbit::{
        hal::{
            clocks::Clocks,
            gpio,
            pwm,
            rtc::{Rtc, RtcInterrupt},
            time::Hertz,
        },
        pac::{self, interrupt},
        board,
    };
    use microbit::hal::timer::Timer;
    use libm::{exp, floorf, sin, sqrtf, ceil, floor};
    use s90::pwm_s90::S90;


    #[init]
    fn setup() -> State {
        defmt::println!("driver test init");
        let board = microbit::Board::take().unwrap();
        let timer = Timer::new(board.TIMER0);
        let mut p0_02 = board.pins.p0_02.into_push_pull_output(gpio::Level::High);
        let _ = p0_02.set_low();
        let mut pwm0 = pwm::Pwm::new(board.PWM0);
        pwm0
            .set_output_pin(pwm::Channel::C0, p0_02.degrade())
            .set_prescaler(pwm::Prescaler::Div1) //scale 约小精度越高
            .set_period(Hertz(500u32))
            .enable();
        let duty_at_0_degress = (pwm0.get_max_duty() as f64 * 1.0) as u16;
        let duty_at_180_degress = 0;
        pwm0.set_duty(pwm::Channel::C0,duty_at_180_degress);
        //defmt::println!("{},{},{}",duty_at_0_degress,duty_at_180_degress,state.pwm.get_duty(super::pwm::Channel::C0));
        let  s90 = S90::new(pwm0,
                                      pwm::Channel::C0,
                                      duty_at_0_degress,
                                      duty_at_180_degress,
                                      false).unwrap();

        let mut p0_03 = board.pins.p0_03.into_push_pull_output(gpio::Level::High);
        let _ = p0_03.set_low();
        let mut pwm1 = pwm::Pwm::new(board.PWM1);
        pwm1
            .set_output_pin(pwm::Channel::C0, p0_03.degrade())
            .set_prescaler(pwm::Prescaler::Div1) //scale 约小精度越高
            .set_period(Hertz(400u32))
            .enable();
        let duty_at_0_degress = (pwm1.get_max_duty() as f64 * 1.0) as u16;
        let duty_at_180_degress = 0;
        pwm1.set_duty(pwm::Channel::C0,duty_at_0_degress);
        //defmt::println!("{},{},{}",duty_at_0_degress,duty_at_180_degress,state.pwm.get_duty(super::pwm::Channel::C0));
        let  s90_revert = S90::new(pwm1,
                                      pwm::Channel::C0,
                                      duty_at_0_degress,
                                      duty_at_180_degress,
                                      true).unwrap();

        State { s90,s90_revert,timer }
    }

    #[test]
    fn simple_action(state:&mut State) {
        // let duty_at_0_degress = (state.pwm.get_max_duty() as f64 * 1.0) as u16;
        // let duty_at_180_degress = 0;
        // state.pwm.set_duty(super::pwm::Channel::C0,0);
        // defmt::println!("{},{},{}",duty_at_0_degress,duty_at_180_degress,state.pwm.get_duty(super::pwm::Channel::C0));
        // let mut s90 = super::S90::new(&mut state.pwm,
        //                           super::pwm::Channel::C0,
        //                           duty_at_0_degress,
        //                           duty_at_180_degress,
        //                           false).unwrap();

        defmt::println!("dgXXX----:{}",state.s90.read().0);
        defmt::println!("simple_action");
        let d = state.s90.read().0;
        defmt::println!("dg0----:{}",d);
        animate(
            &mut [
                Move::new(
                    &mut state.s90,
                    180.0.degrees()
                ),
            ],
            2000,
            &mut state.timer,
        );
        let d = state.s90.read().0;
        defmt::println!("dg1----:{}",d);
        assert_eq!(180, ceil(d) as u16);
    }

    // #[test]
    // fn simple_action_revert(state:&mut State) {
    //     let duty_at_0_degress = (state.pwm.get_max_duty() as f64 * 1.0) as u16;
    //     let duty_at_180_degress = 0;
    //     state.pwm.set_duty(super::pwm::Channel::C0,duty_at_0_degress);
    //     defmt::println!("{},{},{}",duty_at_0_degress,duty_at_180_degress,state.pwm.get_duty(super::pwm::Channel::C0));
    //     let mut s90 = super::S90::new(&mut state.pwm,
    //                                   super::pwm::Channel::C0,
    //                                   duty_at_0_degress,
    //                                   duty_at_180_degress,
    //                                   true).unwrap();
    //     defmt::println!("dgXXX----:{}",s90.read().0);
    //     defmt::println!("simple_action_revert");
    //     let d = s90.read().0;
    //     defmt::println!("dg0----:{}",d);
    //     animate(
    //         &mut [
    //             Move::new(
    //                 &mut s90,
    //                 179.0.degrees()
    //             ),
    //         ],
    //         2000,
    //         &mut state.timer,
    //     );
    //     let d = s90.read().0;
    //     defmt::println!("dg1----:{}",d);
    //     assert_eq!(179, floor(d) as u16);
    // }
}
