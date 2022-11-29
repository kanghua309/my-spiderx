#![no_main]
#![no_std]
#![allow(dead_code, unused_imports)]

use defmt_rtt as _;
use panic_probe as _;
use microbit::Board;
use microbit::hal::timer::Timer;
use microbit::pac::TIMER0;
use microbit::hal::pwm::Pwm;

use s90::{F64Ext, Servo, S90};

struct State {
    s90:S90<Pwm<microbit::hal::pac::PWM0>,microbit::hal::pwm::Channel>,
    s90_revert:S90<Pwm<microbit::hal::pac::PWM1>,microbit::hal::pwm::Channel>,
    timer:Timer<TIMER0>,
}

#[defmt_test::tests]
mod tests {
    use core::time::Duration;
    use defmt::{assert_eq, unwrap};
    use microbit::{Board, pac, hal};
    use my_board::MyBoard;
    use animate::{animate, Move};
    use s90::{F64Ext, Servo, S90};
    use embedded_hal::Pwm;
    use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
    use microbit::{
        hal::{
            pwm,
            time::Hertz,
            timer,
        },
    };
    use super::State;

    #[init]
    fn setup() -> State {
        defmt::println!("test init");
        let mut my_board = MyBoard::take().unwrap();
        let duty_at_0_degress = (my_board.servo_pwms.pwm0.get_max_duty() as f64 * 1.0) as u16;
        let duty_at_180_degress = 0;
        my_board.servo_pwms.pwm0.set_duty(pwm::Channel::C0,duty_at_180_degress);
        //defmt::println!("{},{},{}",duty_at_0_degress,duty_at_180_degress,state.pwm.get_duty(super::pwm::Channel::C0));
        let  s90 = S90::new(my_board.servo_pwms.pwm0,
                            pwm::Channel::C0,
                            duty_at_0_degress,
                            duty_at_180_degress,
                            false).unwrap();

        let duty_at_0_degress = (my_board.servo_pwms.pwm1.get_max_duty() as f64 * 1.0) as u16;
        let duty_at_180_degress = 0;
        my_board.servo_pwms.pwm1.set_duty(pwm::Channel::C0,duty_at_0_degress);
        //defmt::println!("{},{},{}",duty_at_0_degress,duty_at_180_degress,state.pwm.get_duty(super::pwm::Channel::C0));
        let  s90_revert = S90::new(my_board.servo_pwms.pwm1,
                            pwm::Channel::C0,
                            duty_at_0_degress,
                            duty_at_180_degress,
                            true).unwrap();
        State{ s90, s90_revert, timer: my_board.timer0}
    }

    #[test]
    fn simple_test(state: &mut State) {
        defmt::println!("simple_test");

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
    }
}