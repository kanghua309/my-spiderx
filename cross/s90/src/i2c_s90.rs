#![cfg_attr(not(test), no_std)]

use  embedded_hal::blocking::i2c::{Write, Read, WriteRead};
use crate::{Degrees, degrees_to_duty, DriverError, duty_to_degrees, Servo};
use defmt_rtt as _;
use crate::DriverError::{PwmError, UnknownError};


pub struct S90<'a,T>
    where
    T: embedded_hal::blocking::i2c::Write + embedded_hal::blocking::i2c::Read + embedded_hal::blocking::i2c::WriteRead
{
    pub i2c:&'a mut T,
    address:u8,
    chan:u8,
    init_angle:Degrees,
    cur_angle:Degrees,
    inverted: bool,
}


impl<T> S90<'_,T>
    where
        T: embedded_hal::blocking::i2c::Write + embedded_hal::blocking::i2c::Read + embedded_hal::blocking::i2c::WriteRead
{
    pub fn new(i2c:&mut T,address:u8,chan:u8,init_angle:Degrees,inverted: bool) -> Result<S90<'_,T>, DriverError> {
        let driver = S90 {i2c, address, chan, init_angle, cur_angle: init_angle, inverted};
        let real_angle = {
            if inverted {
                Degrees(180.0 - init_angle.0)
            }
            else{
                init_angle
            }
        };
        driver.i2c.write(address, &[chan, real_angle.0 as u8]);
        Ok(driver)
    }
    pub fn destroy(self) -> Self{
        self
    }
}

impl<T> Servo for S90<'_,T>
    where
        T: embedded_hal::blocking::i2c::Write + embedded_hal::blocking::i2c::Read + embedded_hal::blocking::i2c::WriteRead
{
    fn read(&self) -> Degrees {
        match self.inverted {
            true  => Degrees(180.0 - self.cur_angle.0),
            false => self.cur_angle,
        }
    }

    fn write(&mut self, degrees: Degrees) -> () {
        match self.inverted {
            true  => {
                self.cur_angle = Degrees(180.0 - degrees.0);
            },
            false => {
                self.cur_angle = degrees;
            },
        }
        self.i2c.write(self.address, &[self.chan, self.cur_angle.0 as u8]).unwrap_or(())
    }
}