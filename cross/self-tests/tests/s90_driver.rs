#![no_main]
#![no_std]
#![allow(dead_code, unused_imports)]

use defmt_rtt as _;
use panic_probe as _;
use s90::S90;

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

    #[init]
    fn setup() -> State  {
        defmt::println!("driver test init");

        let board = microbit::Board::take().unwrap();
        let timer = Timer::new(board.TIMER0);
        let mut pin = board.pins.p0_02.into_push_pull_output(super::gpio::Level::High);
        let _ = pin.set_low();
        // Use the PWM peripheral to generate a waveform for the speaker
        let  pwm = super::pwm::Pwm::new(board.PWM0);
        pwm
            // output the waveform on the speaker pin
            .set_output_pin(super::pwm::Channel::C0, pin.degrade())
            // Use prescale by 16 to achive darker sounds
            .set_prescaler(super::pwm::Prescaler::Div1)
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
    fn simple_set_duty(state :&mut State) {
        defmt::println!("driver_simple_set_duty");
        let duty_at_0_degress = (state.pwm.get_max_duty() as f64 * 1.0) as u16;
        let duty_at_180_degress = 0;
        state.pwm.set_duty(super::pwm::Channel::C0,0);
        defmt::println!("{},{},{}",duty_at_0_degress,duty_at_180_degress,state.pwm.get_duty(super::pwm::Channel::C0));
        let mut s90 = super::S90::new(&mut state.pwm,
                                      super::pwm::Channel::C0,
                                      duty_at_0_degress,
                                      duty_at_180_degress,
                                      false).unwrap();

        for i in 0..10 {
            defmt::println!("loop");
            //defmt::println!("dg0:{}",state.s90.read().0);
            // state.s90.write(0.0.degrees());
            state.timer.delay_ms(1000u32);
            let dg = (i as f64 * 10.0) as f64;
            s90.write(dg.degrees());
            defmt::println!("dg:{}",s90.read().0);
            let d = s90.read().0;
            assert_eq!(i * 10, ceil(d) as u16);
        }
    }

}