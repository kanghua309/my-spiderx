#![cfg_attr(not(test), no_std)]
//
// // use core::marker::PhantomData;
// use embedded_hal::Pwm;
// // use embedded_hal::digital::v2::OutputPin;
// use defmt_rtt as _;

#[derive(Debug)]
pub enum DriverError {
    PwmError,
    UnknownError,
}

pub struct Degrees(pub f64);
pub trait F64Ext {
    fn degrees(self) -> Degrees;
}
impl F64Ext for f64 {
    fn degrees(self) -> Degrees {
        if self > 180.0 || self < 0.0 {
            panic!("Invalid angle");
        }
        Degrees(self)
    }
}

pub trait Servo {
    fn read(&self) -> Degrees;
    fn write(&mut self, degrees: Degrees) -> ();
}

pub fn degrees_to_duty(
    duty_at_0_degrees: u16,
    duty_at_180_degrees: u16,
    degrees: Degrees,
    inverted: bool,
) -> u16 {
    let norm_degrees = if inverted {
        180.0 - degrees.0
    } else {
        degrees.0
    };
    (duty_at_180_degrees as f64
        + ((norm_degrees / 180.0)
        * (duty_at_0_degrees - duty_at_180_degrees) as f64)) as u16
}

pub fn duty_to_degrees(
    duty_at_0_degrees: u16,
    duty_at_180_degrees: u16,
    duty: u16,
    inverted: bool,
) -> Degrees {
    let norm_degrees = ((duty_at_0_degrees - duty) as f64
        / (duty_at_0_degrees - duty_at_180_degrees) as f64)
        * 180.0;
    if inverted {
        Degrees(180.0 - norm_degrees)
    } else {
        Degrees(norm_degrees)
    }
}