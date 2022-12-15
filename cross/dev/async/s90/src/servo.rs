use core::future::Future;

#[derive(Debug)]
pub enum DriverError {
    PwmError,
    UnknownError,
}

#[derive(Clone, Copy)]
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

// pub trait Servo {
//     type ReadFuture<'m>: Future<Output = Degrees> where Self: 'm;
//     fn read<'m>(&'m self) -> Self::ReadFuture<'m>;
//
//     type WriteFuture<'m>: Future<Output = ()> where Self: 'm;
//     fn write<'m>(&'m mut self, degrees: Degrees) -> Self::WriteFuture<'m>;
// }

pub trait Servo {
    async fn read<'m>(&'m self) -> Degrees;
    async fn write<'m>(&'m mut self, degrees: Degrees) -> ();
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
    (duty_at_0_degrees as f64
        + ((norm_degrees / 180.0)
        * (duty_at_180_degrees - duty_at_0_degrees) as f64)) as u16
}

pub fn duty_to_degrees(
    duty_at_0_degrees: u16,
    duty_at_180_degrees: u16,
    duty: u16,
    inverted: bool,
) -> Degrees {
    let norm_degrees = ((duty_at_180_degrees - duty) as f64
        / (duty_at_180_degrees - duty_at_0_degrees) as f64)
        * 180.0;
    if inverted {
        Degrees(180.0 - norm_degrees)
    } else {
        Degrees(norm_degrees)
    }
}