#![no_std]
#![no_main]
mod display_driver;
mod linedriver;

use core::u32;

use cortex_m::asm::delay;
use cortex_m::singleton;
use embedded_hal::digital::OutputPin;
use pio_proc::pio_file;
use rp2040_hal::dma::{single_buffer, Channel, DMAExt};
use rp2040_hal::gpio::{FunctionPio0, Pin};
use rp2040_hal::pio::PinDir;
use rp_pico::hal::{clocks::init_clocks_and_plls, pac, pio::PIOExt, sio::Sio, watchdog::Watchdog};
use rp_pico::Pins;
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

    let linedriver=linedriver::LineDriver::new(pins.gpio10.into_function(),pins.gpio16.into_function(),pins.gpio18.into_function(),pins.gpio20.into_function(),pins.gpio22.into_function(), pins.gpio13.into_function());
    let r1:Pin<_, FunctionPio0, _>=pins.gpio2.into_function();
    let clk:Pin<_, FunctionPio0, _>=pins.gpio11.into_function();
    let latch:Pin<_, FunctionPio0, _>=pins.gpio12.into_function();

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    // Import the PIO program
    let program = pio_file!("src/hub75line.pio");

    let installed = pio.install(&program.program).unwrap();
    let (mut sm, _, mut tx) = rp2040_hal::pio::PIOBuilder::from_program(installed)
        //.out_pins(2, 3) // R1,G1,B1 on pins 2,3,4
        .set_pins(r1.id().num, 1) // CLK on pin 5
        .side_set_pin_base(clk.id().num) // LATCH on pin 4
        .clock_divisor_fixed_point(1, 0)
        .build(sm0);
    sm.set_pindirs([(r1.id().num, PinDir::Output),(clk.id().num, PinDir::Output),(latch.id().num, PinDir::Output)]);
    
    sm.start();
    loop {
        tx.write(u32::MAX);
        delay(125000);
    }
}
