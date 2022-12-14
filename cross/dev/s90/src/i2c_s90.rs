#![cfg_attr(not(test), no_std)]

use  embedded_hal::blocking::i2c::{Write, Read, WriteRead};
use crate::{Degrees, degrees_to_duty, DriverError, duty_to_degrees, Servo};
use defmt_rtt as _;
use crate::DriverError::{PwmError, UnknownError};


pub struct S90<'a,T>
    where
    T: embedded_hal::blocking::i2c::Write + embedded_hal::blocking::i2c::Read + embedded_hal::blocking::i2c::WriteRead
{
    pub i2c:&'a T,
    address:u8,
    chan:u8,
    //init_angle:Degrees,
    cur_angle:Degrees,
    inverted: bool,
}


impl<T> S90<'_,T>
    where
        T: embedded_hal::blocking::i2c::Write + embedded_hal::blocking::i2c::Read + embedded_hal::blocking::i2c::WriteRead
{
    pub fn new(i2c:& T,address:u8,chan:u8,inverted: bool) -> Result<S90<'_,T>, DriverError> {
        let driver = S90 {i2c, address, chan,cur_angle:Degrees(0.0), inverted};
        // let real_angle = {
        //     if inverted {
        //         Degrees(180.0 - init_angle.0)
        //     }
        //     else{
        //         init_angle
        //     }
        // };
        // unsafe {
        //     let mutable_t: &mut T = &mut *(i2c as *const T as *mut T);
        //     let _ = mutable_t.write(address, &[chan, real_angle.0 as u8]);
        // }
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
        // match self.inverted {
        //     true  => Degrees(180.0 - self.cur_angle.0),
        //     false => self.cur_angle,
        // }
        self.cur_angle
    }

    fn write(&mut self, degrees: Degrees) -> () {
        let real_angle = {
            match self.inverted {
                true => {
                    Degrees(180.0 - degrees.0)
                },
                false => {
                    degrees
                }
            }
        };

        unsafe {
            let mutable_t: &mut T = &mut *(self.i2c as *const T as *mut T);
            let _ = mutable_t.write(self.address, &[self.chan, real_angle.0 as u8]);
        }
        self.cur_angle = degrees
        //self.i2c.write(self.address, &[self.chan, self.cur_angle.0 as u8]).unwrap_or(())
    }
}