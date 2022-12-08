#![cfg_attr(not(test), no_std)]

use  embedded_hal::blocking::i2c::{Write,Read,WriteRead};
use crate::{Degrees, degrees_to_duty, DriverError, duty_to_degrees, Servo};
use defmt_rtt as _;


pub struct S90<'a,T>
    where
    T: embedded_hal::blocking::i2c::Write + embedded_hal::blocking::i2c::Read + embedded_hal::blocking::i2c::WriteRead
{
    pub i2c:&'a mut T,
    address:u8,
    chan:u8,
    duty_at_0_degrees: u16,
    duty_at_180_degrees: u16,
    inverted: bool,
}


impl<T> S90<'_,T>
    where
        T: embedded_hal::blocking::i2c::Write + embedded_hal::blocking::i2c::Read + embedded_hal::blocking::i2c::WriteRead
{
    pub fn new(i2c:&mut T,address:u8,chan:u8,duty_at_0_degrees:u16,duty_at_180_degrees:u16 ,inverted: bool) -> Result<S90<'_,T>, DriverError> {
        let driver = S90 {i2c, address, chan, duty_at_0_degrees, duty_at_180_degrees, inverted};
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
        duty_to_degrees(
            self.duty_at_0_degrees,
            self.duty_at_180_degrees,
            //self.i2c.read(self.address, &mut [self.chan]).unwrap(),
            0u16,//TODO
            self.inverted,
        )
    }

    fn write(&mut self, degrees: Degrees) -> () {
        // let dg = degrees_to_duty(
        //     self.duty_at_0_degrees,
        //     self.duty_at_180_degrees,
        //     degrees,
        //     self.inverted,
        // );
        //defmt::println!("Hello, world!---------------:{}",dg);
        self.i2c.write(self.address, &[self.chan,degrees.0 as u8]);
        ()
    }
}