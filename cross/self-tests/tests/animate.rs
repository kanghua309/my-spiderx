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

use microbit::hal::timer::Timer;
use microbit::pac::TIMER0;

struct State {
    s90: S90<microbit::hal::pwm::Pwm<microbit::hal::pac::PWM0>,pwm::Channel>,
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
    use libm::{exp, floorf, sin, sqrtf,ceil};
    use animate::{animate, Move};

    #[init]
    fn setup() -> State {
        defmt::println!("driver test init");
        // let cm_periph = unwrap!(cortex_m::Peripherals::take());
        // Board::init(cm_periph.DCB, cm_periph.DWT)
        let board = microbit::Board::take().unwrap();
        let mut timer = Timer::new(board.TIMER0);

        //let mut xpin = board.pins.p0_02.into_push_pull_output(super::gpio::Level::High);
        let mut pin = board.pins.p0_02.into_push_pull_output(super::gpio::Level::High);
        let _ = pin.set_low();
        // Use the PWM peripheral to generate a waveform for the speaker
        let mut pwm = super::pwm::Pwm::new(board.PWM0);
        pwm
            // output the waveform on the speaker pin
            .set_output_pin(super::pwm::Channel::C0, pin.degrade())
            // Use prescale by 16 to achive darker sounds
            .set_prescaler(super::pwm::Prescaler::Div32)
            // Initial frequency
            .set_period(super::Hertz(500u32))
            // Configure for up and down counter mode
            //.set_counter_mode(super::pwm::CounterMode::UpAndDown)
            // Set maximum duty cycle
            //.set_max_duty(6554)
            // enable PWM
            .enable();
        // pwm
        //     .set_seq_refresh(super::pwm::Seq::Seq0, 0)
        //     .set_seq_end_delay(super::pwm::Seq::Seq0, 0);
        //TODO:*******************************************************************/
        let duty_at_0_degress = (pwm.get_max_duty() as f64 * 1.0) as u16;
        let duty_at_180_degress = 0;
        pwm.set_duty(super::pwm::Channel::C0,1);
        //TODO:********************************************************************/

        defmt::println!("{},{},{}",duty_at_0_degress,duty_at_180_degress,pwm.get_duty(super::pwm::Channel::C0));
        let s90 = super::S90::new(pwm,
                                  super::pwm::Channel::C0,
                                  duty_at_0_degress,
                                  duty_at_180_degress).unwrap();
        State { s90,timer }
    }

    #[test]
    fn simple_action(state:&mut State) {
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
        assert_ne!(0, ceil(d) as u16);

    }

}
