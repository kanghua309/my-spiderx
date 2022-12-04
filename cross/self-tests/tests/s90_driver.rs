#![no_main]
#![no_std]
#![allow(dead_code, unused_imports)]

use defmt_rtt as _;
use panic_probe as _;
use s90::S90;

use microbit::hal::pwm::Pwm;
use microbit::hal::timer::Timer;
use microbit::pac::TIMER0;

struct State {
    s90:S90<Pwm<microbit::hal::pac::PWM0>,microbit::hal::pwm::Channel>,
    timer:Timer<TIMER0>,
}

#[defmt_test::tests]
mod tests {
    use core::time::Duration;
    use defmt::{assert_eq, unwrap};
    use s90::{F64Ext, Servo, S90};
    use super::State;
    use libm::{exp, floorf, sin, sqrtf, ceil, floor};
    use embedded_hal::Pwm;
    use embedded_hal::digital::v2::OutputPin;
    use embedded_hal::prelude::{_embedded_hal_blocking_delay_DelayMs, _embedded_hal_blocking_spi_Write};
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

    #[init]
    fn setup() -> State  {
        defmt::println!("driver test init");
        let board = microbit::Board::take().unwrap();
        let timer = Timer::new(board.TIMER0);
        let mut pin = board.pins.p0_02.into_push_pull_output(gpio::Level::High);
        let _ = pin.set_low();
        // Use the PWM peripheral to generate a waveform for the speaker
        let mut pwm = pwm::Pwm::new(board.PWM0);
        pwm
            .set_output_pin(pwm::Channel::C0, pin.degrade())
            .set_prescaler(pwm::Prescaler::Div1)
            .set_period(Hertz(400u32))
            .enable();
        let duty_at_0_degress = (pwm.get_max_duty() as f64 * 1.0) as u16;
        let duty_at_180_degress = 0;
        pwm.set_duty(pwm::Channel::C0,0);
        defmt::println!("{},{},{}",duty_at_0_degress,duty_at_180_degress,pwm.get_duty(pwm::Channel::C0));
        let s90 = S90::new(pwm,
                           pwm::Channel::C0,
                           duty_at_0_degress,
                           duty_at_180_degress,
                           false).unwrap();
        State { s90,timer }
    }

    #[test]
    fn simple_set_duty(state :&mut State) {
        defmt::println!("driver_simple_set_duty");
        for i in 0..10 {
            defmt::println!("loop");
            //defmt::println!("dg0:{}",state.s90.read().0);
            state.timer.delay_ms(1000u32);
            let dg = (i as f64 * 10.0) as f64;
            state.s90.write(dg.degrees());
            defmt::println!("dg:{}",state.s90.read().0);
            let d = state.s90.read().0;
            assert_eq!(i * 10, ceil(d) as u16);
        }
    }

}