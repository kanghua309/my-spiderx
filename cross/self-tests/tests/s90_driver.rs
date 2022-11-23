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

struct State {
    //s90: S90<gpio::Pin<gpio::Output<gpio::PushPull>>,microbit::hal::pwm::Pwm<microbit::hal::pac::PWM0>,pwm::Channel>,
    s90: S90<microbit::hal::pwm::Pwm<microbit::hal::pac::PWM0>,pwm::Channel>,
}

#[defmt_test::tests]
mod tests {
    use core::time::Duration;
    use defmt::{assert_eq, unwrap};
    use embedded_hal::digital::v2::OutputPin;
    use s90::{F64Ext, Servo};
    // use embedded_hal::Pwm;
    // use embedded_hal::digital::v2::OutputPin;
    use super::State;

    #[init]
    fn setup() -> State {
        defmt::println!("driver test init");
        // let cm_periph = unwrap!(cortex_m::Peripherals::take());
        // Board::init(cm_periph.DCB, cm_periph.DWT)
        let board = microbit::Board::take().unwrap();
        //let mut xpin = board.pins.p0_02.into_push_pull_output(super::gpio::Level::High);
        let mut pin = board.pins.p0_01.into_push_pull_output(super::gpio::Level::High);
        let _ = pin.set_low();
        // Use the PWM peripheral to generate a waveform for the speaker
        let pwm = super::pwm::Pwm::new(board.PWM0);
        pwm
            // output the waveform on the speaker pin
            .set_output_pin(super::pwm::Channel::C0, pin.degrade())
            // Use prescale by 16 to achive darker sounds
            .set_prescaler(super::pwm::Prescaler::Div4)
            // Initial frequency
            .set_period(super::Hertz(1u32))
            // Configure for up and down counter mode
            .set_counter_mode(super::pwm::CounterMode::UpAndDown)
            // Set maximum duty cycle
            .set_max_duty(6554)
            // enable PWM
            .enable();
        pwm
            .set_seq_refresh(super::pwm::Seq::Seq0, 0)
            .set_seq_end_delay(super::pwm::Seq::Seq0, 0);
        
        let s90 = super::S90::new(pwm, super::pwm::Channel::C0).unwrap();
        State { s90 }
    }

    #[test]
    fn simple_set_duty(state :&mut State) {
        defmt::println!("driver_simple_set_duty");
        state.s90.write(180.0.degrees());
        let degree = state.s90.read();
        assert_eq!(180.0,degree.0);
    }

    // #[test]
    // fn simple_rotate(pwm: &mut impl Pwm<Duty = u16,Channel = pwm::Channel>) {
    //     defmt::println!("simple_rotate");
    //     // const EXPECTED: [u8; 2] = [3, 66];
    //     // assert_eq!(EXPECTED, board.scd30.get_firmware_version().unwrap())
    //     pwm.set_duty(pwm::Channel::C0,3277);
    //     let duty = pwm.get_duty(pwm::Channel::C0);
    //     assert_eq!(3277,duty);
    // }
}