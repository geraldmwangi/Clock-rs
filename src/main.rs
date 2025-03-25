#![no_std]
#![no_main]
mod display_driver;
mod linedriver;
pub mod volatile_access;
pub mod led_matrix;
//  mod volatile_access;

use core::u32;

use cortex_m::asm::delay;
use cortex_m::singleton;
use display_driver::DisplayDriver;
use ds323x::ic::DS3231;
use ds323x::{DateTimeAccess, Ds323x, NaiveDate, Rtcc, Timelike};
use embedded_hal::digital::OutputPin;

use rp2040_hal::dma::{single_buffer, Channel, DMAExt};
use rp2040_hal::gpio::{FunctionPio0, Pin};
use rp2040_hal::pio::PinDir;
use rp_pico::hal::{clocks::init_clocks_and_plls, pac, pio::PIOExt, sio::Sio, watchdog::Watchdog};
use rp_pico::Pins;
use rp_pico::hal::{clocks::ClocksManager, gpio::Error, i2c::I2C,fugit::RateExtU32};
use rp_pico::hal::prelude::*;
use embedded_hal::delay::DelayNs;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const ROWS: usize = HEIGHT / 2;

// #[link_section = ".boot2"]
// #[used]
// pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[rp2040_hal::entry]
fn main() -> ! {
    rtt_init_print!();
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
    
    let clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    

    let core=pac::CorePeripherals::take().unwrap();

    pins.gpio2.into_push_pull_output();
    pins.gpio3.into_push_pull_output();
    pins.gpio4.into_push_pull_output();
    pins.gpio5.into_push_pull_output();
    pins.gpio8.into_push_pull_output();
    pins.gpio9.into_push_pull_output();
    pins.gpio10.into_push_pull_output();
    pins.gpio16.into_push_pull_output();
    pins.gpio18.into_push_pull_output();
    pins.gpio20.into_push_pull_output();
    pins.gpio22.into_push_pull_output();
    pins.gpio11.into_push_pull_output();
    pins.gpio12.into_push_pull_output();
    pins.gpio13.into_push_pull_output();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

     let mut display=DisplayDriver::new( delay).unwrap();

    let sda_pin = pins.gpio6.reconfigure();
    let scl_pin = pins.gpio7.reconfigure();

    let i2c = I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );


    let mut rtc = Ds323x::new_ds3231(i2c);
    let datetime = NaiveDate::from_ymd(2025, 3, 22).and_hms(17, 46, 00);
    rtc.enable();
    //rtc.set_datetime(&datetime);

    // Set the time to 12:34:56
    // rtc.set_time(12, 34, 56).unwrap();

    // Get the current time
    // let (hours, minutes, seconds) = rtc.get_time().unwrap();
    // rprintln!("Current time: {:02}:{:02}:{:02}", hours, minutes, seconds);
    
    loop {
        let date=rtc.time();

        match date {
            Ok(date) => {
                // rprintln!("Current time: {:02}:{:02}:{:02}", date.hour(), date.minute(), date.second());
                display.fill_frame(date);
                display.display_image();
            },
            Err(e) => rprintln!("error: {:#?}",e),
        }
        

      
    }
}
