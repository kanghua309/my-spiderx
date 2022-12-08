#![cfg_attr(not(test), no_std)]

// use core::marker::PhantomData;
use embedded_hal::Pwm;
// use embedded_hal::digital::v2::OutputPin;
use defmt_rtt as _;


pub struct S90<'a,T>{
    pub i2c:&'a mut Twim<T>,
    address:u8,
    chan:u8,
    duty_at_0_degrees: u16,
    duty_at_180_degrees: u16,
    inverted: bool,
}


impl<T> S90<T>
{
    pub fn new(i2c:Twim<T>,address:u8,chan:u8,duty_at_0_degrees:u16,duty_at_180_degrees:u16 ,inverted: bool) -> Result<Self, DriverError> {
        let driver = S90 {i2c, address, chan, duty_at_0_degrees, duty_at_180_degrees, inverted};
        Ok(driver)
    }
    pub fn destroy(self) -> Self{
        self
    }
}

impl<T> Servo for S90<T>
{
    fn read(&self) -> Degrees {
        duty_to_degrees(
            self.duty_at_0_degrees,
            self.duty_at_180_degrees,
            self.i2c.read(self.address,&[self.chan]).unwrap(),
            self.inverted,
        )
    }

    fn write(&mut self, degrees: Degrees) -> () {
        let dg = degrees_to_duty(
            self.duty_at_0_degrees,
            self.duty_at_180_degrees,
            degrees,
            self.inverted,
        );
        //defmt::println!("Hello, world!---------------:{}",dg);
        self.i2c.write(self.address,&[self.chan,degrees_to_duty])
    }
}