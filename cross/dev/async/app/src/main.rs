
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


mod spider;

use core::borrow::BorrowMut;
use core::mem;
use embassy_executor::Spawner;
use embassy_nrf::{interrupt, twim};
use embassy_nrf::twim::Twim;
use embassy_time::{Duration, Timer, Delay};
use {defmt_rtt as _, panic_probe as _};
use defmt::{info, *};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use nrf_softdevice::ble::{gatt_server, peripheral};
use nrf_softdevice::{raw, Softdevice};
use crate::spider::Spider;

#[derive(Debug)]
enum CrawlAction {
    front,
    back,
}

static CHANNEL: Channel<ThreadModeRawMutex, CrawlAction, 1> = Channel::new();
#[nrf_softdevice::gatt_service(uuid = "9e7312e0-2354-11eb-9f10-fbc30a62cf38")]
struct FooService {
    #[characteristic(uuid = "9e7312e0-2354-11eb-9f10-fbc30a63cf38", read, write, notify, indicate)]
    foo: u16,
}
#[nrf_softdevice::gatt_server]
struct Server {
    foo: FooService,
}

#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await
}

#[embassy_executor::task]
async fn spider_receive_task() -> ! {
    let p = embassy_nrf::init(Default::default());
    info!("Initializing TWI...");
    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    let mut twi = Twim::new(p.TWISPI0, irq, p.P1_00, p.P0_26, config);
    let mut spider= Spider::new(&twi);
    spider.init().await;
    info!("Spider Init Over... ");
    info!("Spider Wait Cmd");
    loop {
        match CHANNEL.recv().await {
            CrawlAction::front => {
                info!("Get Cmd - Front");
                spider.walk_forward(&mut Delay,1,1000).await;
            },
            CrawlAction::back =>  {
                info!("Get Cmd - Back");
                spider.walk_backward(&mut Delay,1,1000).await;
            },
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let config = nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_RC as u8,
            rc_ctiv: 4,
            rc_temp_ctiv: 2,
            accuracy: 7,
        }),
        conn_gap: Some(raw::ble_gap_conn_cfg_t {
            conn_count: 6,
            event_length: 24,
        }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 256 }),
        gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t { attr_tab_size: 32768 }),
        gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
            adv_set_count: 1,
            periph_role_count: 3,
            central_role_count: 3,
            central_sec_count: 0,
            _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
        }),
        gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
            p_value: b"SpiderCtrl" as *const u8 as _,
            current_len: 10,
            max_len: 10,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(raw::BLE_GATTS_VLOC_STACK as u8),
        }),
        ..Default::default()
    };
    let sd = Softdevice::enable(&config);
    let server = unwrap!(Server::new(sd));
    unwrap!(spawner.spawn(softdevice_task(sd)));
    info!("spawn soft-device over");
    unwrap!(spawner.spawn(spider_receive_task()));
    info!("spawn spider-task over");

    #[rustfmt::skip]
        let adv_data = &[
        0x02, 0x01, raw::BLE_GAP_ADV_FLAGS_LE_ONLY_GENERAL_DISC_MODE as u8,
        0x03, 0x03, 0x09, 0x18,
        0x0a, 0x09, b'H', b'e', b'l', b'l', b'o', b'R', b'u', b's', b't',
    ];
    #[rustfmt::skip]
        let scan_data = &[
        0x03, 0x03, 0x09, 0x18,
    ];
    loop {
        let config = peripheral::Config::default();
        let adv = peripheral::ConnectableAdvertisement::ScannableUndirected { adv_data, scan_data };
        let conn = unwrap!(peripheral::advertise_connectable(sd, adv, &config).await);
        info!("advertising done!");
        let res = gatt_server::run(&conn, &server,  |e| match e {
            ServerEvent::Foo(e) => match e {
                FooServiceEvent::FooWrite(val) => {
                    info!("read cmd: {}", val);
                    match val {
                        1 => {CHANNEL.try_send(CrawlAction::front);},
                        2 => {CHANNEL.try_send(CrawlAction::back);},
                        _ => { warn!("Illegal action") }
                    }
                },
                _ => { warn!("Illegal action")},
            },
        })
            .await;
        if let Err(e) = res {
            info!("gatt_server run exited with error: {:?}", e);
        }
    }
}