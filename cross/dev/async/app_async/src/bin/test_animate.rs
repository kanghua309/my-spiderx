#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::interrupt;
use embassy_nrf::twim::{self, Twim};
use embassy_time::{Delay};

use {defmt_rtt as _, panic_probe as _};
use animate::{animate, Move};
use s90::i2c_s90::S90;
use s90::{Servo};
use s90::F64Ext;

const ADDRESS: u8 = 0x80;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    info!("Initializing TWI...");
    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    let twi = Twim::new(p.TWISPI0, irq, p.P1_00, p.P0_26, config);
    let mut s90 = S90::new(&twi,
                           ADDRESS,
                           0,
                           false).unwrap();
    //info!("dg0----");
    animate(
        &mut [
            Move::new(
                &mut s90,
                180.0_f64.degrees()
            ),
        ],
        2000,
        &mut Delay
    ).await;
    let _d = s90.read().await.0;
    //info!("dg1----:{}",d);
}