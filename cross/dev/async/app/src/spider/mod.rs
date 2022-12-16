mod walk;
mod look;
mod turn;


use core::marker::PhantomData;
use defmt::unwrap;
use embassy_nrf::pwm::Instance;
use embassy_nrf::twim::Twim;
use embassy_time::Delay;
use s90::i2c_s90::S90;
use s90::{Degrees, Servo};
use crate::spider::walk::walk;

pub struct Spider<'a,T>
where
      T: embassy_nrf::twim::Instance,
{
    hip_front_left: S90<'a, Twim<'a,T>>,
    hip_front_right: S90<'a, Twim<'a,T>>,
    hip_rear_left: S90<'a, Twim<'a,T>>,
    hip_rear_right: S90<'a, Twim<'a,T>>,
    knee_front_left: S90<'a, Twim<'a,T>>,
    knee_front_right: S90<'a, Twim<'a,T>>,
    knee_rear_left:  S90<'a, Twim<'a,T>>,
    knee_rear_right: S90<'a, Twim<'a,T>>,
}

impl<'a,T> Spider<'a,T> where
    T: embassy_nrf::twim::Instance,
{
    pub fn new(i2c:&'a Twim<'a,T>) -> Self {
        let hip_front_left  =  S90::new(i2c, 0x80, 0, false).unwrap();
        let hip_front_right =  S90::new(i2c, 0x80, 1, true).unwrap();
        let hip_rear_left  =   S90::new(i2c, 0x80, 2, true).unwrap();
        let hip_rear_right =   S90::new(i2c, 0x80, 3, false).unwrap();
        let knee_front_left =  S90::new(i2c, 0x80, 15, false).unwrap();
        let knee_front_right =  S90::new(i2c, 0x80, 14, true).unwrap();
        let knee_rear_left =  S90::new(i2c, 0x80, 13, true).unwrap();
        let knee_rear_right =  S90::new(i2c, 0x80, 12, false).unwrap();
        Spider{
            hip_front_left,
            hip_front_right,
            hip_rear_left,
            hip_rear_right,
            knee_front_left,
            knee_front_right,
            knee_rear_left,
            knee_rear_right,
        }
    }
    pub async fn init(&mut self) {
        self.hip_front_left.write(Degrees(45.0)).await;
        self.hip_front_right.write(Degrees(45.0)).await;
        self.hip_rear_left.write(Degrees(45.0)).await;
        self.hip_rear_right.write(Degrees(45.0)).await;
        self.knee_front_left.write(Degrees(165.0)).await;
        self.knee_front_right.write(Degrees(165.0)).await;
        self.knee_rear_left.write(Degrees(165.0)).await;
        self.knee_rear_right.write(Degrees(165.0)).await;
    }

    pub async fn walk_forward(&mut self, sleep: &mut Delay, steps: u32, speed: u32) {
        walk(
            (&mut self.knee_front_left, &mut self.knee_rear_right, ),
            (&mut self.knee_rear_left, &mut self.knee_front_right),
            (&mut self.hip_front_left,&mut self.hip_rear_right),
            (&mut self.hip_rear_left,&mut self.hip_front_right),
            sleep,
            steps,
            speed,
        ).await;
    }

    pub async fn walk_backward(&mut self, sleep: &mut Delay, steps: u32, speed: u32) {
        walk(
            ( &mut self.knee_front_left, &mut self.knee_rear_right,),
            (&mut self.knee_rear_left, &mut self.knee_front_right),
            ( &mut self.hip_rear_right, &mut self.hip_front_left),
            ( &mut self.hip_front_right,&mut self.hip_rear_left),
            sleep,
            steps,
            speed,
        ).await;
    }
    //
    // pub async fn walk_left(&self, sleep: &mut Delay, steps: u32, speed: u32) {
    //     walk(
    //         (self.knee_rear_right, self.knee_front_left),
    //         (self.knee_rear_left, self.knee_front_right),
    //         (self.hip_front_left, self.hip_rear_right),
    //         (self.hip_front_right, self.hip_rear_left),
    //         sleep,
    //         steps,
    //         speed,
    //     ).await;
    // }
    //
    // pub async fn walk_right(&self, sleep: &mut Delay, steps: u32, speed: u32) {
    //     walk(
    //         (self.knee_rear_right, self.knee_front_left),
    //         (self.knee_rear_left, self.knee_front_right),
    //         (self.hip_rear_right, self.hip_front_left),
    //         (self.hip_rear_left, self.hip_front_right),
    //         sleep,
    //         steps,
    //         speed,
    //     ).await;
    // }
}