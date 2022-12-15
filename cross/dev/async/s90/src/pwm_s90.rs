

use core::future::Future;
// use core::marker::PhantomData;
use embedded_hal::Pwm;
// use embedded_hal::digital::v2::OutputPin;
use defmt_rtt as _;
use crate::servo::{Degrees, degrees_to_duty, DriverError, duty_to_degrees, Servo};

pub struct S90<PWM, CH> {
    //pin: PIN,
    pub pwm:PWM,
    //_mark: PhantomData<C>,
    chan:CH,
    duty_at_0_degrees: u16,
    duty_at_180_degrees: u16,
    inverted: bool,
}


impl <PWM,CH> S90<PWM,CH>
    where
        PWM: embedded_hal::Pwm<Channel=CH, Duty=u16>,
        CH:  Copy,
{
    pub fn new(pwm:PWM, chan:CH,duty_at_0_degrees:u16,duty_at_180_degrees:u16 ,inverted: bool) -> Result<Self, DriverError> {
        let driver = S90 {pwm, chan, duty_at_0_degrees, duty_at_180_degrees, inverted};
        Ok(driver)
    }
    pub fn destroy(self) -> Self{
        self
    }
}

impl<PWM,CH> Servo for S90<PWM,CH>
    where
        PWM: Pwm<Channel=CH, Duty=u16>,
        CH: Copy,
{
    async fn read(&self) -> Degrees {
            duty_to_degrees(
                self.duty_at_0_degrees,
                self.duty_at_180_degrees,
                self.pwm.get_duty(self.chan),
                self.inverted,
            )
    }

    async fn write(&mut self, degrees: Degrees) -> () {
        let dg = degrees_to_duty(
            self.duty_at_0_degrees,
            self.duty_at_180_degrees,
            degrees,
            self.inverted,
        );
        self.pwm.set_duty(self.chan, dg);
        //defmt::println!("Hello, world2!---------------:{}",dg)
    }
}