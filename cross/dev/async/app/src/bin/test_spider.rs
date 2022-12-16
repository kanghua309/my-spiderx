#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[path = "../spider/mod.rs"]
mod spider;

use core::borrow::BorrowMut;
use embassy_executor::Spawner;
use embassy_nrf::{interrupt, twim};
use embassy_nrf::twim::Twim;
use embassy_time::{Duration, Timer, Delay};
use {defmt_rtt as _, panic_probe as _};
use defmt::{info, *};
use nrf_softdevice::ble::{gatt_server, peripheral};
use nrf_softdevice::{raw, Softdevice};
use crate::spider::Spider;

const ADDRESS: u8 = 0x80;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    info!("Initializing TWI...");
    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    let mut twi = Twim::new(p.TWISPI0, irq, p.P1_00, p.P0_26, config);
    let mut spider= Spider::new(&twi);
    spider.init().await;
    //spider.walk_forward(&mut Delay,100,2000).await;
    spider.walk_backward(&mut Delay,10,2000).await;
}