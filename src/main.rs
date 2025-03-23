#![no_std]
#![no_main]

use {defmt_rtt as _, embassy_executor::Spawner, embassy_rp::gpio::{Level, Output}, panic_probe as _}; // global logger
use defmt::{error, warn, info, debug, trace};
use embassy_rp::{bind_interrupts, i2c::{self, Config, InterruptHandler}, peripherals::I2C1};

bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
});
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    
    
    let p=embassy_rp::init(Default::default());
    let sda=p.PIN_6;
    let scl=p.PIN_7;
    info!("set up i2c ");
    let mut i2c = i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, Config::default());

    // let led = Output::new(p.PIN_13, Level::Low);
    // unwrap!(spawner.spawn(blinker(led, Duration::from_millis(300))));
    // spawner.spawn(blinker(led, Duration::from_millis(300))).unwrap();
}