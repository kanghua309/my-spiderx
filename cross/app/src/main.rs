#![no_std]
#![no_main]
#[warn(unused_imports)]
//
// use core::slice;
//
// use board::{Board, Instant, Scd30, Serial};
use defmt::unwrap;
use defmt_rtt as _;
// use heapless::{consts, Vec};
// use messages::{Host2Target, Measurement, Target2Host};
use panic_probe as _;
// use rtic::cyccnt::U32Ext;

use cortex_m_rt::entry;

use core::time::Duration;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use defmt::{assert_eq};
// use embedded_hal::Pwm;
// use embedded_hal::digital::v2::OutputPin;
// use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
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
use microbit::hal::prelude::{_embedded_hal_Pwm, OutputPin};
use microbit::hal::timer::Timer;
#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");
    // let cm_periph = unwrap!(cortex_m::Peripherals::take());
    // Board::init(cm_periph.DCB, cm_periph.DWT)
    let board = microbit::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut pin = board.pins.p0_02.into_push_pull_output(gpio::Level::High);
    let _ = pin.set_low();
    // Use the PWM peripheral to generate a waveform for the speaker
    let mut pwm = pwm::Pwm::new(board.PWM0);
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

    loop{
        defmt::println!("loop");
        timer.delay_ms(1000u32);
        pwm.set_duty(pwm::Channel::C0,3277);
        //timer.delay_ms(1000u32);
        //pwm.set_duty(pwm::Channel::C0,6554);
        let duty = pwm.get_duty(pwm::Channel::C0);
        defmt::println!("?{}",duty);
    }
}

// #[rtic::app(device = board::pac, monotonic = rtic::cyccnt::CYCCNT)]
// const APP: () = {
//     struct Resources {
//         #[init(0)]
//         count: u32,
//         scd30: Scd30,
//         serial: Serial,
//         #[init(None)]
//         measurement: Option<Measurement>,
//     }
//
//     #[init(spawn = [periodic])]
//     fn init(cx: init::Context) -> init::LateResources {
//         let mut board = Board::init(cx.core.DCB, cx.core.DWT);
//
//         board
//             .scd30
//             .start_continuous_measurement()
//             .unwrap();
//         unwrap!(cx.spawn.periodic());
//
//         defmt::info!("DONE");
//         init::LateResources {
//             scd30: board.scd30,
//             serial: board.serial,
//         }
//     }
//
//     #[idle(resources = [serial, measurement])]
//     fn idle(mut cx: idle::Context) -> ! {
//         let serial = cx.resources.serial;
//         let mut rx_buffer = Vec::<u8, consts::U64>::new();
//         let mut tx_buffer = [0; 64];
//
//         let mut byte = 0;
//         loop {
//             serial.read(slice::from_mut(&mut byte)).unwrap();
//
//             if byte == 0 {
//                 defmt::info!("RX bytes={}", &*rx_buffer);
//
//                 if let Ok(request) = postcard::from_bytes_cobs::<Host2Target>(&mut rx_buffer) {
//                     match request {
//                         Host2Target::GetLastMeasurement => {
//                             let resp = cx
//                                 .resources
//                                 .measurement
//                                 .lock(|opt| opt.clone())
//                                 .map(Target2Host::Measurement)
//                                 .unwrap_or(Target2Host::NotReady);
//
//                             let bytes = postcard::to_slice_cobs(&resp, &mut tx_buffer).unwrap();
//                             defmt::info!("TX bytes={}", bytes);
//                             serial.write(bytes).unwrap();
//                         }
//                     }
//                 } else {
//                     defmt::error!("postcard deserialization error")
//                 }
//
//                 rx_buffer.clear();
//             } else {
//                 rx_buffer.push(byte).unwrap();
//             }
//         }
//     }
//
//     // NOTE instead of a periodic software task it would be more efficient to use a hardware task
//     // bound to an "external pin interrupt" that fires when the SCD30's RDY pin goes high
//     #[task(schedule = [periodic], resources = [count, scd30, measurement])]
//     fn periodic(cx: periodic::Context) {
//         // run this again in 20 ms -- this polling period affects the `timestamp` accuracy
//         unwrap!(cx.schedule.periodic(cx.scheduled + 1_280_000.cycles()));
//
//         let scd30 = cx.resources.scd30;
//         if let Ok(data_ready) = scd30.data_ready() {
//             // NOTE likewise this timestamp would be more accurate if a hardware task was used
//             let timestamp = Instant::now().as_cycles();
//
//             if data_ready {
//                 if let Ok(sensor_data) = scd30.read_measurement() {
//                     defmt::info!("{}", sensor_data);
//
//                     *cx.resources.measurement = Some(Measurement {
//                         id: *cx.resources.count,
//                         timestamp,
//                         co2: sensor_data.co2,
//                     });
//                     *cx.resources.count += 1;
//                 } else {
//                     defmt::error!("couldn't read sensor data");
//                 }
//             }
//         } else {
//             defmt::error!("couldn't check sensor's data ready flag");
//         }
//     }
//
//     extern "C" {
//         fn RTC0();
//     }
// };
