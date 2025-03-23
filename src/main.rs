#![no_std]
#![no_main]

use {defmt_rtt as _, embassy_executor::Spawner, embassy_rp::gpio::{Level, Output}, panic_probe as _}; // global logger
use defmt::unwrap;
use defmt::{debug, error, expect, info, trace, warn};
use ds323x::Timelike;
use ds323x::{Rtcc,ic::DS3231, interface::I2cInterface, Ds323x};
use embassy_rp::{bind_interrupts, i2c::{self, Config, I2c, InterruptHandler}, peripherals::I2C1};
use embassy_time::{Duration, Timer};

bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
});

#[embassy_executor::task]
async fn rtc_task(mut rtc: Ds323x<I2cInterface<I2c<'static, I2C1, embassy_rp::i2c::Async>>, DS3231>){
    loop {
        let time=rtc.time();
        Timer::after_millis(500).await;
        
        match time{
            Ok(time) => {
                info!("Current time {}:{}:{}",time.hour(),time.minute(),time.second());
            },
            Err(e) => error!("Error reading rtc chip"),
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    
    
    let p=embassy_rp::init(Default::default());
    let sda=p.PIN_6;
    let scl=p.PIN_7;
    info!("set up i2c ");
    // debug!("set up i2c ");
    let mut i2c = i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, Config::default());
    let rtc: Ds323x<I2cInterface<I2c<'_, I2C1, embassy_rp::i2c::Async>>, DS3231>=ds323x::Ds323x::new_ds3231(i2c);

    unwrap!(spawner.spawn(rtc_task(rtc)));


    

    // let led = Output::new(p.PIN_13, Level::Low);
    // unwrap!(spawner.spawn(blinker(led, Duration::from_millis(300))));
    // spawner.spawn(blinker(led, Duration::from_millis(300))).unwrap();
}