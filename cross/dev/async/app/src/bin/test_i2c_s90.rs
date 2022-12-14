#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::interrupt;
use embassy_nrf::twim::{self, Twim};
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};
use s90::i2c_s90::S90;
use s90::{Degrees, Servo};

const ADDRESS: u8 = 0x80;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    info!("Initializing TWI...");
    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    let mut twi = Twim::new(p.TWISPI0, irq, p.P1_00, p.P0_26, config);

    let mut s90 = S90::new(&twi,
                           0x80,
                           0,
                           false).unwrap();

    let mut buf = [0u8; 2];
    loop {
        for index in 0..1 {
            info!("index-:{}",index);
            for angle in 0..180 {
                let mut acc = [index, angle];
                //unwrap!(twi.blocking_write(ADDRESS, &mut acc));
                s90.write(Degrees(angle as f64)).await;
                //info!("Write: {=[u8]:x} | Read: {=[u8]:x}", acc, buf);
                Timer::after(Duration::from_millis(100)).await;
                info!("angle-:{}",angle);
            }
        }
    }
}