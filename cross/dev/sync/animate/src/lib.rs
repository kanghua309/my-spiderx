#![cfg_attr(not(test), no_std)]
#![allow(dead_code, unused_imports)]

//extern crate heapless;

use defmt_rtt as _;
use panic_probe as _;
use s90::pwm_s90::S90;

use embedded_hal::blocking::delay;

use heapless::Vec;
use heapless::consts::*;
use s90::{Degrees, F64Ext, Servo};

pub struct Move<'a> {
    servo: &'a mut dyn Servo,
    desired: Degrees,
}

impl<'a> Move<'a> {
    pub fn new(servo: &'a mut dyn Servo, desired: Degrees) -> Move<'a> {
        Move { servo, desired }
    }
}

pub fn animate(moves: &mut [Move], speed: u32, sleep: &mut impl delay::DelayMs<u32>) -> () {
    const STEP_SPEED: u32 = 20;
    let step_num = speed / STEP_SPEED;
    let mut deltas: Vec<_, U8> = Vec::new();
    for m in &mut *moves {
        let current = m.servo.read();
        let delta = (m.desired.0 - current.0) / step_num as f64;
        deltas.push(delta).unwrap();
    }
    for _ in 0..(speed / STEP_SPEED) {
        for i in 0..moves.len() {
            let m = &mut moves[i];
            let delta = deltas[i];

            let new_degrees = m.servo.read().0 + delta;
            let d = new_degrees.degrees();
            m.servo.write(d);
        }
        sleep.delay_ms(STEP_SPEED);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
    }
}
