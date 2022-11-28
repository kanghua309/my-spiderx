#![no_std]

use core::borrow::BorrowMut;
use defmt::unwrap;
use defmt_rtt as _;

use panic_probe as _;
use cortex_m_rt::entry;

use microbit::{hal::{
    clocks::Clocks,
    gpio,
    pwm,
    rtc::{Rtc, RtcInterrupt},
    time::{Hertz},
}, pac::{self, interrupt}, board, Board};
use microbit::hal::gpio::{Level, Output, p0, p1, PushPull};
use microbit::hal::gpio::p0::{P0_02, P0_03};
use microbit::hal::pwm::{Channel, Pwm};

pub struct ServoPwms {
    pub Pwm1: Pwm<microbit::hal::pac::PWM0>,
    //pub pin2: P0_03<Output<PushPull>>,
}

pub struct MyBoard{
    pub servo_pwms:ServoPwms,
    pub timer1:pac::TIMER1,
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
        let p1parts = p1::Parts::new(p.P1);
        let pwm1 = Pwm::new(p.PWM0);
        let pin1 = p0parts.p0_02.into_push_pull_output(Level::High);
        pwm1
            // output the waveform on the speaker pin
            .set_output_pin(Channel::C0, pin1.degrade())
            // Use prescale by 16 to achive darker sounds
            .set_prescaler(pwm::Prescaler::Div1)
            // Initial frequency
            .set_period(Hertz(500u32))
            // Configure for up and down counter mode
            //.set_counter_mode(super::pwm::CounterMode::UpAndDown)
            // Set maximum duty cycle
            //.set_max_duty(6554)
            // enable PWM
            .enable();

        let board = Self {
            servo_pwms: ServoPwms {
                Pwm1: pwm1,
            },
            timer1: p.TIMER1,
        };
        board
    }
}