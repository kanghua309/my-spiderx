#![no_main]
#![no_std]
#![allow(dead_code, unused_imports)]

use defmt_rtt as _;
use panic_probe as _;

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
            // prelude::OutputPin,
            pwm,
            rtc::{Rtc, RtcInterrupt},
            time::Hertz,
        },
        pac::{self, interrupt},
        board,
    };
    use microbit::hal::timer::Timer;

    //
    // struct State {
    //     S90: S90<PIN,PWM>,
    // }


    #[init]
    fn setup() -> impl Pwm<Duty = u16,Channel = pwm::Channel> {
        defmt::println!("test init");
        // let cm_periph = unwrap!(cortex_m::Peripherals::take());
        // Board::init(cm_periph.DCB, cm_periph.DWT)
        let board = microbit::Board::take().unwrap();
        // let mut timer = Timer::new(board.TIMER0);

        let mut pin = board.pins.p0_02.into_push_pull_output(gpio::Level::High);
        let _ = pin.set_low();
        // Use the PWM peripheral to generate pwm waveform for the speaker
        let pwm = pwm::Pwm::new(board.PWM0);
        pwm
            // output the waveform on the speaker pin
            .set_output_pin(pwm::Channel::C0, pin.degrade())
            // Use prescale by 16 to achive darker sounds
            .set_prescaler(pwm::Prescaler::Div32)
            // Initial frequency
            .set_period(Hertz(400u32))
            // Configure for up and down counter mode
            //.set_counter_mode(pwm::CounterMode::UpAndDown)
            // Set maximum duty cycle
            //.set_max_duty(6554)
            // enable PWM
            .enable();
        // pwm
        //     .set_seq_refresh(pwm::Seq::Seq0, 0)
        //     .set_seq_end_delay(pwm::Seq::Seq0, 0);
        pwm
    }

    #[test]
    fn simple_set_duty(pwm: &mut impl Pwm<Duty = u16,Channel = pwm::Channel>) {
        defmt::println!("simple_set_duty");
        defmt::println!("duty {}", pwm.get_duty(pwm::Channel::C0));
        assert_eq!(pwm.get_max_duty(),pwm.get_duty(pwm::Channel::C0));
        pwm.set_duty(pwm::Channel::C0,250);
        defmt::println!("duty {}", pwm.get_duty(pwm::Channel::C0));
        assert_eq!(pwm.get_max_duty()-250,pwm.get_duty(pwm::Channel::C0));
    }
}
