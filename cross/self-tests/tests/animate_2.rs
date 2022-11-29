#![no_main]
#![no_std]
#![allow(dead_code, unused_imports)]

use defmt_rtt as _;
use panic_probe as _;
use s90::S90;
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
use core::option::Option::{Some,None};

struct State {
    pwm: core::option::Option<Pwm<microbit::hal::pac::PWM0>>,
    // s90:S90<Pwm<microbit::hal::pac::PWM0>,microbit::hal::pwm::Channel>,
    // s90_revert:S90<Pwm<microbit::hal::pac::PWM1>,microbit::hal::pwm::Channel>,
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
    use s90::{F64Ext, Servo, S90};
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
    use core::ptr::replace;

    #[init]
    fn setup() -> State {
        defmt::println!("driver test init");
        let board = microbit::Board::take().unwrap();
        let timer = Timer::new(board.TIMER0);
        let mut p0_02 = board.pins.p0_02.into_push_pull_output(gpio::Level::High);
        let _ = p0_02.set_low();
        let pwm0 = pwm::Pwm::new(board.PWM0);
        pwm0
            .set_output_pin(pwm::Channel::C0, p0_02.degrade())
            .set_prescaler(pwm::Prescaler::Div1) //scale 约小精度越高
            .set_period(Hertz(500u32))
            .enable();
        State { pwm:Some(pwm0), timer }
    }

    #[test]
    fn simple_action(state:&mut State) {
        let duty_at_0_degress = (state.pwm.as_ref().unwrap().get_max_duty() as f64 * 1.0) as u16;
        let duty_at_180_degress = 0;
        state.pwm.as_mut().unwrap().set_duty(pwm::Channel::C0,0);
        defmt::println!("{},{},{}",duty_at_0_degress,duty_at_180_degress,state.pwm.as_ref().unwrap().get_duty(pwm::Channel::C0));
        let my_pwm = unsafe {
            replace(&mut state.pwm, None)
        };
        let mut s90 = super::S90::new(my_pwm.unwrap(),
                                  pwm::Channel::C0,
                                  duty_at_0_degress,
                                  duty_at_180_degress,
                                  false).unwrap();

        defmt::println!("dgXXX----:{}",s90.read().0);
        defmt::println!("simple_action");
        let d = s90.read().0;
        defmt::println!("dg0----:{}",d);
        animate(
            &mut [
                Move::new(
                    &mut s90,
                    180.0.degrees()
                ),
            ],
            2000,
            &mut state.timer,
        );
        let d = s90.read().0;
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
