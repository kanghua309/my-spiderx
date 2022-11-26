#![no_main]
#![no_std]
#![allow(dead_code, unused_imports)]

use defmt_rtt as _;
use panic_probe as _;
use s90::S90;
use animate::*;

//use embedded_hal::Pwm;
//use embedded_hal::digital::v2::OutputPin;
use microbit::{
    hal::{
        clocks::Clocks,
        gpio,
        prelude::OutputPin,
        pwm,
        rtc::{Rtc, RtcInterrupt},
        time::Hertz,
    },
    pac::{self, interrupt},
    board,
};
use microbit::hal::pwm::Pwm;

use microbit::hal::timer::Timer;
use microbit::pac::TIMER0;

struct State {
    pwm: Pwm<microbit::hal::pac::PWM0>,
    timer:Timer<TIMER0>,
}

#[defmt_test::tests]
mod tests {
    use core::time::Duration;
    use defmt::{assert_eq, unwrap};
    use embedded_hal::digital::v2::OutputPin;
    use embedded_hal::prelude::{_embedded_hal_blocking_delay_DelayMs, _embedded_hal_blocking_spi_Write};
    use embedded_hal::Pwm;
    use microbit::hal::Timer;
    use s90::{F64Ext, Servo};
    use super::State;
    use libm::{exp, floorf, sin, sqrtf, ceil, floor};
    use animate::{animate, Move};

    #[init]
    fn setup() -> State {
        defmt::println!("driver test init");

        let board = microbit::Board::take().unwrap();
        let timer = Timer::new(board.TIMER0);
        let mut pin = board.pins.p0_02.into_push_pull_output(super::gpio::Level::High);
        let _ = pin.set_low();
        // Use the PWM peripheral to generate a waveform for the speaker
        let pwm = super::pwm::Pwm::new(board.PWM0);
        pwm
            // output the waveform on the speaker pin
            .set_output_pin(super::pwm::Channel::C0, pin.degrade())
            // Use prescale by 16 to achive darker sounds
            .set_prescaler(super::pwm::Prescaler::Div1) //scale 约小精度越高
            // Initial frequency
            .set_period(super::Hertz(500u32))
            // Configure for up and down counter mode
            //.set_counter_mode(super::pwm::CounterMode::UpAndDown)
            // Set maximum duty cycle
            //.set_max_duty(6554)
            // enable PWM
            .enable();
        State { pwm,timer }
    }

    #[test]
    fn simple_action(state:&mut State) {
        let duty_at_0_degress = (state.pwm.get_max_duty() as f64 * 1.0) as u16;
        let duty_at_180_degress = 0;
        state.pwm.set_duty(super::pwm::Channel::C0,0);
        defmt::println!("{},{},{}",duty_at_0_degress,duty_at_180_degress,state.pwm.get_duty(super::pwm::Channel::C0));
        let mut s90 = super::S90::new(&mut state.pwm,
                                  super::pwm::Channel::C0,
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

    #[test]
    fn simple_action_revert(state:&mut State) {
        let duty_at_0_degress = (state.pwm.get_max_duty() as f64 * 1.0) as u16;
        let duty_at_180_degress = 0;
        state.pwm.set_duty(super::pwm::Channel::C0,duty_at_0_degress);
        defmt::println!("{},{},{}",duty_at_0_degress,duty_at_180_degress,state.pwm.get_duty(super::pwm::Channel::C0));
        let mut s90 = super::S90::new(&mut state.pwm,
                                      super::pwm::Channel::C0,
                                      duty_at_0_degress,
                                      duty_at_180_degress,
                                      true).unwrap();
        defmt::println!("dgXXX----:{}",s90.read().0);
        defmt::println!("simple_action_revert");
        let d = s90.read().0;
        defmt::println!("dg0----:{}",d);
        animate(
            &mut [
                Move::new(
                    &mut s90,
                    179.0.degrees()
                ),
            ],
            2000,
            &mut state.timer,
        );
        let d = s90.read().0;
        defmt::println!("dg1----:{}",d);
        assert_eq!(179, floor(d) as u16);
    }
}
