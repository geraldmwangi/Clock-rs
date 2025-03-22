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
use embedded_hal::digital::OutputPin;

use rp2040_hal::dma::{single_buffer, Channel, DMAExt};
use rp2040_hal::gpio::{FunctionPio0, Pin};
use rp2040_hal::pio::PinDir;
use rp_pico::hal::{clocks::init_clocks_and_plls, pac, pio::PIOExt, sio::Sio, watchdog::Watchdog};
use rp_pico::Pins;
use rp_pico::hal::{clocks::ClocksManager, gpio::Error, pac, i2c::I2C};
use rp_pico::hal::prelude::*;
use embedded_hal::blocking::delay::DelayMs;
use crate::ds3231::Ds3231;
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
    let mut display=DisplayDriver::new(pins, core, clocks).unwrap();

    let sda_pin = pins.gpio0.into_mode::<rp_pico::hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio1.into_mode::<rp_pico::hal::gpio::FunctionI2C>();

    let i2c = I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let mut rtc = Ds3231::new(i2c);

    // Set the time to 12:34:56
    rtc.set_time(12, 34, 56).unwrap();

    // Get the current time
    let (hours, minutes, seconds) = rtc.get_time().unwrap();
    println!("Current time: {:02}:{:02}:{:02}", hours, minutes, seconds);
    
    loop {
        display.fill_frame();
        display.display_image();
      
    }
}
