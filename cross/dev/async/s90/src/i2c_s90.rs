

use core::future::Future;
use embedded_hal_async::*;
use crate::{Degrees, degrees_to_duty, DriverError, duty_to_degrees, Servo};
use defmt_rtt as _;
use crate::DriverError::{PwmError, UnknownError};


pub struct S90<'a,T>
    where
    T: embedded_hal_async::i2c::I2c
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
        T: embedded_hal_async::i2c::I2c
{
    pub fn new(i2c:& T,address:u8,chan:u8,inverted: bool) -> Result<S90<'_,T>, DriverError> {
        let driver = S90 {i2c, address, chan,cur_angle:Degrees(0.0), inverted};
        Ok(driver)
    }
    pub fn destroy(self) -> Self{
        self
    }
}

impl<T> Servo for S90<'_,T>
    where
        T: embedded_hal_async::i2c::I2c
{
    async fn read<'m>(&'m self) -> Degrees{
            self.cur_angle
    }

    async fn write<'m>(&'m mut self, degrees: Degrees) -> (){
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
        //TODO : very trick
        unsafe {
            let mutable_t: &mut T = &mut *(self.i2c as *const T as *mut T);
            let _ = mutable_t.write(self.address, &[self.chan, real_angle.0 as u8]).await;
        }
        self.cur_angle = degrees;
    }
}