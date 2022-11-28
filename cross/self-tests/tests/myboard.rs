#![no_main]
#![no_std]
#![allow(dead_code, unused_imports)]
#![feature(const_option)]

use defmt_rtt as _;
use microbit::Board;
use panic_probe as _;
use my_board::MyBoard;




#[defmt_test::tests]
mod tests {
    use core::time::Duration;
    use defmt::{assert_eq, unwrap};
    use microbit::{Board, pac};
    // use microbit::{board, hal::{
    //     clocks::Clocks,
    //     gpio,
    //     // prelude::OutputPin,
    //     pwm,
    //     rtc::{Rtc, RtcInterrupt},
    //     time::Hertz,
    // }, pac::{self, interrupt}};
    use my_board::MyBoard;


    #[init]
    fn setup() -> MyBoard {
        defmt::println!("test init");
        //let my_board = MyBoard::take().unwrap();
        let my_board = MyBoard::take().unwrap();
        my_board
    }

    #[test]
    fn simple_test(my_board: &mut MyBoard) {
        defmt::println!("simple_test");
        // defmt::println!("duty {}", pwm.get_duty(pwm::Channel::C0));
        // assert_eq!(pwm.get_max_duty(),pwm.get_duty(pwm::Channel::C0));
        // pwm.set_duty(pwm::Channel::C0,250);
        // defmt::println!("duty {}", pwm.get_duty(pwm::Channel::C0));
        // assert_eq!(pwm.get_max_duty()-250,pwm.get_duty(pwm::Channel::C0));
    }
}