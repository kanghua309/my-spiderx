#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::*;
// use embassy_nrf::pwm::{Prescaler, SimplePwm};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
use s90::pwm_s90::S90;
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
use microbit::hal::prelude::{_embedded_hal_Pwm, OutputPin};
use s90::Degrees;
use s90::Servo;
use s90::F64Ext;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("test init");
    let _ = embassy_nrf::init(Default::default());
    let board = microbit::Board::take().unwrap();
    // let mut timer = Timer::new(board.TIMER0);
    let mut pin = board.pins.p0_02.into_push_pull_output(gpio::Level::High);
    let _ = pin.set_low();
    // Use the PWM peripheral to generate pwm waveform for the speaker
    let mut pwm = pwm::Pwm::new(board.PWM0);
    pwm
        .set_output_pin(pwm::Channel::C0, pin.degrade())
        // Use prescale by 16 to achive darker sounds
        .set_prescaler(pwm::Prescaler::Div128)
        .set_max_duty(2500)
        // enable PWM
        .enable();
    info!("pwm initialized!");
    // 1ms 0deg (1/.008=125), 1.5ms 90deg (1.5/.008=187.5), 2ms 180deg (2/.008=250),
    let duty_at_0_degress = 2500 - 250;
    let duty_at_180_degress = 2500 - 150;
    pwm.set_duty(pwm::Channel::C0,duty_at_0_degress);
    info!("{},{},{}",duty_at_0_degress,duty_at_180_degress,pwm.get_duty(pwm::Channel::C0));
    let mut s90 = S90::new(pwm,
                       pwm::Channel::C0,
                       duty_at_0_degress,
                       duty_at_180_degress,
                       false).unwrap();
    loop {
        for index in 0..1 {
            info!("index-:{}",index);
            for angle in 0..180 {
                let mut acc = [index, angle];
                info!("angle-:{}",angle);
                s90.write(Degrees(angle as f64)).await;
                Timer::after(Duration::from_millis(100)).await;
            }
        }
    }
}
