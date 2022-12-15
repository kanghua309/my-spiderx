#![cfg_attr(not(test), no_std)]
#![allow(dead_code, unused_imports)]

use defmt_rtt as _;
//use panic_probe as _;
use s90::servo;
use defmt::*;

use embedded_hal_async::delay;
use embedded_hal_async::delay::DelayUs;

use heapless::Vec;
use heapless::consts::*;
use s90::{Degrees, F64Ext,Servo};

pub struct Move<'a,S> 
    where S: Servo
{
    servo: &'a mut S,
    desired: Degrees,
}

impl<'a,S> Move<'a,S> where S: Servo {
    pub fn new(servo: &'a mut S, desired: Degrees) -> Self {
        Move { servo, desired }
    }
}

pub async fn animate<S:Servo> (moves: &mut [Move<'_,S>], speed: u32, sleep: &mut impl delay::DelayUs) -> () {
    // defmt::println!("Hello, world1!---------------:");
    const STEP_SPEED: u32 = 20;
    let step_num = speed / STEP_SPEED;
    let mut deltas: Vec<_, U8> = Vec::new();
    for m in &mut *moves {
        let current = m.servo.read().await;
        let delta = (m.desired.0 - current.0) / step_num as f64;
        deltas.push(delta).unwrap();
    }
    for j in 0..(speed / STEP_SPEED) {
        for i in 0..moves.len() {
            let m = &mut moves[i];
            let delta = deltas[i];

            let new_degrees = m.servo.read().await.0 + delta;
            let d = new_degrees.degrees();
            //defmt::println!("Hello, world2!---------------:{}:{}:{}",speed / STEP_SPEED,j,d.0);
            m.servo.write(d).await;
        }
        sleep.delay_ms(STEP_SPEED).await;
    }
    //defmt::println!("Hello, world2!---------------:")
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//     }
// }
