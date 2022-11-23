#![no_main]
#![no_std]
#![allow(dead_code, unused_imports)]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use core::time::Duration;
    use defmt::{assert_eq, unwrap};
    use embedded_hal::Pwm;
    use embedded_hal::digital::v2::OutputPin;
    use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
    use microbit::{
        hal::{
            clocks::Clocks,
            gpio,
            // prelude::OutputPin,
            pwm,
            rtc::{Rtc, RtcInterrupt},
            time::Hertz,
        },
        pac::{self, interrupt},
        board,
    };
    use microbit::hal::timer::Timer;

    //
    // struct State {
    //     S90: S90<PIN,PWM>,
    // }


    #[init]
    fn setup() -> impl Pwm<Duty = u16,Channel = pwm::Channel> {
        defmt::println!("test init");

        // let cm_periph = unwrap!(cortex_m::Peripherals::take());
        // Board::init(cm_periph.DCB, cm_periph.DWT)
        let board = microbit::Board::take().unwrap();
        let mut timer = Timer::new(board.TIMER0);

        let mut pin = board.pins.p0_02.into_push_pull_output(gpio::Level::High);
        let _ = pin.set_low();
        // Use the PWM peripheral to generate a waveform for the speaker
        let pwm = pwm::Pwm::new(board.PWM0);
        pwm
            // output the waveform on the speaker pin
            .set_output_pin(pwm::Channel::C0, pin.degrade())
            // Use prescale by 16 to achive darker sounds
            .set_prescaler(pwm::Prescaler::Div4)
            // Initial frequency
            .set_period(Hertz(1u32))
            // Configure for up and down counter mode
            .set_counter_mode(pwm::CounterMode::UpAndDown)
            // Set maximum duty cycle
            .set_max_duty(6554)
            // enable PWM
            .enable();
        pwm
            .set_seq_refresh(pwm::Seq::Seq0, 0)
            .set_seq_end_delay(pwm::Seq::Seq0, 0);
        pwm
    }

    #[test]
    fn simple_set_duty(pwm: &mut impl Pwm<Duty = u16,Channel = pwm::Channel>) {
        defmt::println!("simple_set_duty");
        //let mut timer = Timer::new(board.TIMER0);
        loop{
            defmt::println!("loop");
            //timer.delay_ms(1000u32);
            pwm.set_duty(pwm::Channel::C0,3277);
            //timer.delay_ms(1000u32);
            //pwm.set_duty(pwm::Channel::C0,6554);
            let duty = pwm.get_duty(pwm::Channel::C0);
            defmt::println!("?{}",duty);
        }
        // const EXPECTED: [u8; 2] = [3, 66];
        // assert_eq!(EXPECTED, board.scd30.get_firmware_version().unwrap())
        //pwm.set_duty(pwm::Channel::C0,3277);
        //let duty = pwm.get_duty(pwm::Channel::C0);
        //assert_eq!(3277,duty);
    }

    // #[test]
    // fn simple_rotate(pwm: &mut impl Pwm<Duty = u16,Channel = pwm::Channel>) {
    //     defmt::println!("simple_rotate");
    //     // const EXPECTED: [u8; 2] = [3, 66];
    //     // assert_eq!(EXPECTED, board.scd30.get_firmware_version().unwrap())
    //     pwm.set_duty(pwm::Channel::C0,3277);
    //     let duty = pwm.get_duty(pwm::Channel::C0);
    //     assert_eq!(3277,duty);
    // }
}
//
// #[defmt_test::tests]
// mod tests {
//     use core::time::Duration;
//
//     use board::Board;
//     use defmt::{assert_eq, unwrap};
//
//     #[init]
//     fn init() -> Board {
//         let cm_periph = unwrap!(cortex_m::Peripherals::take());
//         Board::init(cm_periph.DCB, cm_periph.DWT)
//     }
//
//     #[test]
//     fn confirm_firmware_version(board: &mut Board) {
//         const EXPECTED: [u8; 2] = [3, 66];
//
//         assert_eq!(EXPECTED, board.scd30.get_firmware_version().unwrap())
//     }
//
//     #[test]
//     fn data_ready_within_two_seconds(board: &mut Board) {
//         board
//             .scd30
//             .start_continuous_measurement()
//             .unwrap();
//
//         // do this twice because there may be a cached measurement
//         // (the sensor is never power-cycled / reset)
//         for _ in 0..2 {
//             board.delay(Duration::from_millis(2_100));
//             assert!(board.scd30.data_ready().unwrap());
//
//             // clear data ready flag
//             let _ = board.scd30.read_measurement();
//         }
//     }
//
//     #[test]
//     fn reasonable_co2_value(board: &mut Board) {
//         // range reported by the sensor when using I2C
//         const MIN_CO2: f32 = 0.;
//         const MAX_CO2: f32 = 40_000.;
//
//         // do this twice for good measure
//         for _ in 0..2 {
//             while !board.scd30.data_ready().unwrap() {}
//
//             let measurement = board.scd30.read_measurement().unwrap();
//             assert!(measurement.co2.is_finite());
//             assert!(measurement.co2 >= MIN_CO2);
//             assert!(measurement.co2 <= MAX_CO2);
//         }
//     }
// }
