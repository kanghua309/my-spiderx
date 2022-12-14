#![cfg_attr(not(test), no_std)]

// // use core::marker::PhantomData;
// use embedded_hal::Pwm;
// // use embedded_hal::digital::v2::OutputPin;
// use defmt_rtt as _;

pub mod servo;
//pub mod i2c_s90;
pub mod pwm_s90;
pub mod i2c_s90;
pub use servo::*;
//pub use i2c_s90::*;
//pub use pwm_s90::*;


#[rustfmt::skip]
#[cfg(test)]
#[path = "test.rs"]
mod test;
