#![cfg_attr(not(test), no_std)]

use core::marker::PhantomData;
use embedded_hal::Pwm;
use embedded_hal::digital::v2::OutputPin;

// pub enum Channel {
//     C0,
//     C1,
//     C2,
//     C3,
// }

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

pub struct S90<PWM,CH>{
    //pin: PIN,
    pwm: PWM,
    //_mark: PhantomData<C>,
    chan:CH,
    duty_at_0_degrees: u16,
    duty_at_180_degrees: u16,
}


impl <PWM,CH> S90<PWM,CH>
    where
    //PIN: embedded_hal::digital::v2::OutputPin,
    PWM: embedded_hal::Pwm<Channel=CH, Duty=u16>,
    CH: Copy,
{
    pub fn new(pwm:PWM, chan:CH) -> Result<Self, DriverError> {
        let driver = S90 {pwm, chan, duty_at_0_degrees: 3277, duty_at_180_degrees: 6554};
        Ok(driver)
    }
}

impl<PWM,CH> Servo for S90<PWM,CH>
    where
    //PIN: OutputPin,
    PWM: Pwm<Channel=CH, Duty=u16>,
    CH: Copy,
{
    fn read(&self) -> Degrees {
        duty_to_degrees(
            self.duty_at_0_degrees,
            self.duty_at_180_degrees,
            self.pwm.get_duty(self.chan)
        )
    }

    fn write(&mut self, degrees: Degrees) -> () {
        self.pwm.set_duty(self.chan,degrees_to_duty(
            self.duty_at_0_degrees,
            self.duty_at_180_degrees,
            degrees,
        ))
    }
}

pub fn degrees_to_duty(
    duty_at_0_degrees: u16,
    duty_at_180_degrees: u16,
    degrees: Degrees,
) -> u16 {
    (duty_at_0_degrees as f64
        + ((degrees.0 / 180.0)
        * (duty_at_180_degrees - duty_at_0_degrees) as f64)) as u16
}

pub fn duty_to_degrees(
    duty_at_0_degrees: u16,
    duty_at_180_degrees: u16,
    duty: u16,
) -> Degrees {
    Degrees(
        ((duty - duty_at_0_degrees) as f64
            / (duty_at_180_degrees - duty_at_0_degrees) as f64)
            * 180.0,
    )
}



// use crc_any::CRCu8;
// use defmt::Format;
// use embedded_hal::blocking::i2c;
//
// /// SCD30 I2C address
// const ADDRESS: u8 = 0x61;
//
// #[derive(Clone, Copy, Format)]
// pub struct SensorData {
//     pub co2: f32,
//     pub temperature: f32,
//     pub humidity: f32,
// }
//
// /// A SCD30 sensor on the I2C bus `I`
// pub struct Scd30<I>(I)
// where
//     I: i2c::Read + i2c::Write;
//
// /// A driver error
// #[derive(Debug, PartialEq)]
// pub enum Error<E> {
//     /// I2C bus error
//     I2c(E),
//     /// CRC validation failed
//     InvalidCrc,
// }
//
// impl<E, I> Scd30<I>
// where
//     I: i2c::Read<Error = E> + i2c::Write<Error = E>,
// {
//     /// Initializes the SCD30 driver.
//     /// This consumes the I2C bus `I`
//     pub fn init(i2c: I) -> Self {
//         Scd30(i2c)
//     }
//
//     /// Returns the firmware version reported by the SCD30 sensor
//     pub fn get_firmware_version(&mut self) -> Result<[u8; 2], Error<E>> {
//         let command: [u8; 2] = [0xd1, 0x00];
//         let mut rd_buffer = [0u8; 3];
//
//         self.0.write(ADDRESS, &command).map_err(Error::I2c)?;
//         self.0.read(ADDRESS, &mut rd_buffer).map_err(Error::I2c)?;
//
//         let major = rd_buffer[0];
//         let minor = rd_buffer[1];
//         let crc = rd_buffer[2];
//
//         if compute_crc(&rd_buffer[..2]) == crc {
//             Ok([major, minor])
//         } else {
//             Err(Error::InvalidCrc)
//         }
//     }
//
//     pub fn start_continuous_measurement(&mut self) -> Result<(), Error<E>> {
//         // NOTE should be configurable
//         const AMBIENT_PRESSURE: u16 = 1_020;
//
//         // command bytes
//         let mut command: [u8; 5] = [0x00, 0x10, 0x00, 0x00, 0x00];
//         let argument_bytes = &AMBIENT_PRESSURE.to_be_bytes();
//
//         command[2] = argument_bytes[0];
//         command[3] = argument_bytes[1];
//         command[4] = compute_crc(argument_bytes);
//
//         self.0.write(ADDRESS, &command).map_err(Error::I2c)?;
//
//         Ok(())
//     }
//
//     // NOTE testing these 3 methods is left as an exercise for the reader
//     pub fn data_ready(&mut self) -> Result<bool, Error<E>> {
//         let command: [u8; 2] = [0x02, 0x02];
//         let mut rd_buffer = [0u8; 3];
//
//         self.0.write(ADDRESS, &command).map_err(Error::I2c)?;
//         self.0.read(ADDRESS, &mut rd_buffer).map_err(Error::I2c)?;
//
//         Ok(u16::from_be_bytes([rd_buffer[0], rd_buffer[1]]) == 1)
//     }
//
//     pub fn read_measurement(&mut self) -> Result<SensorData, Error<E>> {
//         let command: [u8; 2] = [0x03, 0x00];
//         let mut rd_buffer = [0u8; 18];
//
//         self.0.write(ADDRESS, &command).map_err(Error::I2c)?;
//         self.0.read(ADDRESS, &mut rd_buffer).map_err(Error::I2c)?;
//
//         let data = SensorData {
//             co2: f32::from_bits(u32::from_be_bytes([
//                 rd_buffer[0],
//                 rd_buffer[1],
//                 rd_buffer[3],
//                 rd_buffer[4],
//             ])),
//             temperature: f32::from_bits(u32::from_be_bytes([
//                 rd_buffer[6],
//                 rd_buffer[7],
//                 rd_buffer[9],
//                 rd_buffer[10],
//             ])),
//             humidity: f32::from_bits(u32::from_be_bytes([
//                 rd_buffer[12],
//                 rd_buffer[13],
//                 rd_buffer[15],
//                 rd_buffer[16],
//             ])),
//         };
//         Ok(data)
//     }
//
//     /// Destroys this driver and releases the I2C bus `I`
//     pub fn destroy(self) -> I {
//         self.0
//     }
// }
//
// fn compute_crc(bytes: &[u8]) -> u8 {
//     let mut crc = CRCu8::create_crc(0x31, 8, 0xff, 0x00, false);
//     crc.digest(bytes);
//     crc.get_crc()
// }
//
// #[cfg(test)]
// mod tests {
//     use super::{Error, Scd30, ADDRESS};
//     use embedded_hal_mock::i2c;
//
//     #[test]
//     fn firmware_version() {
//         let expectations = vec![
//             i2c::Transaction::write(ADDRESS, vec![0xD1, 0x00]),
//             i2c::Transaction::read(ADDRESS, vec![0x03, 0x42, 0xF3]),
//         ];
//         let mock = i2c::Mock::new(&expectations);
//
//         let mut scd30 = Scd30::init(mock);
//         let version = scd30.get_firmware_version().unwrap();
//         assert_eq!([3, 66], version);
//
//         let mut mock = scd30.destroy();
//         mock.done(); // verify expectations
//     }
//
//     #[test]
//     fn firmware_version_bad_crc() {
//         let expectations = vec![
//             i2c::Transaction::write(ADDRESS, vec![0xD1, 0x00]),
//             // NOTE negated CRC byte in the response!
//             i2c::Transaction::read(ADDRESS, vec![0x03, 0x42, !0xF3]),
//         ];
//         let mock = i2c::Mock::new(&expectations);
//
//         let mut scd30 = Scd30::init(mock);
//         let res = scd30.get_firmware_version();
//         assert_eq!(Err(Error::InvalidCrc), res);
//
//         scd30.destroy().done(); // verify expectations
//     }
//
//     #[test]
//     fn crc() {
//         // example from the Interface Specification document
//         assert_eq!(super::compute_crc(&[0xBE, 0xEF]), 0x92);
//     }
// }

#[rustfmt::skip]
#[cfg(test)]
#[path = "./test.rs"]
mod test;