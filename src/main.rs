#![no_std]
#![no_main]

use {defmt_rtt as _, embassy_executor::Spawner, embassy_rp::gpio::{Level, Output}, panic_probe as _}; // global logger

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // let p = embassy_nrf::init(Default::default());
    let p=embassy_rp::init(Default::default());

    let led = Output::new(p.PIN_13, Level::Low);
    // unwrap!(spawner.spawn(blinker(led, Duration::from_millis(300))));
    // spawner.spawn(blinker(led, Duration::from_millis(300))).unwrap();
}