#![no_std]

use defmt::unwrap;
use defmt_rtt as _;

use panic_probe as _;

// use microbit::{
//     hal::{
//     pwm,
//     time::{Hertz},
//     },
//     // pac::{self, interrupt}, board, Board
// };
use microbit::hal::gpio::{Level, p0};
use microbit::hal::pwm::{Channel, Pwm};
use microbit::pac;
use microbit::{
    hal::{
        pwm,
        time::Hertz,
    },
};
use microbit::hal::timer::Timer;


pub struct ServoPwms {
    pub pwm0: Pwm<microbit::hal::pac::PWM0>,
    pub pwm1: Pwm<microbit::hal::pac::PWM1>,
}

pub struct MyBoard{
    pub servo_pwms: ServoPwms,
    pub timer0: microbit::hal::timer::Timer<pac::TIMER0>,
}

impl MyBoard {
    pub fn take() -> Option<Self> {
        Some(Self::new(
            pac::Peripherals::take()?,
            pac::CorePeripherals::take()?,
        ))
    }
    pub fn new(p: pac::Peripherals, cp: pac::CorePeripherals) -> Self {
        let p0parts = p0::Parts::new(p.P0);
        //let p1parts = p1::Parts::new(p.P1);
        let pwm0 = Pwm::new(p.PWM0);
        let p0_02 = p0parts.p0_02.into_push_pull_output(Level::High);
        pwm0
            .set_output_pin(Channel::C0, p0_02.degrade())
            .set_prescaler(pwm::Prescaler::Div1)
            .set_period(Hertz(500u32))
            .enable();

        let pwm1 = Pwm::new(p.PWM1);
        let p0_03 = p0parts.p0_03.into_push_pull_output(Level::High);
        pwm1
            .set_output_pin(Channel::C0, p0_03.degrade())
            .set_prescaler(pwm::Prescaler::Div1)
            .set_period(Hertz(500u32))
            .enable();
        let board = Self {
            servo_pwms: ServoPwms {
                pwm0,
                pwm1,
            },
            timer0:  Timer::new(p.TIMER0),
        };
        board
    }
}