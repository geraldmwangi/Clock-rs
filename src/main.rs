#![no_std]
#![no_main]
use core::u32;

use embedded_hal::digital::OutputPin;
use rp2040_hal::dma::{DMAExt,Channel};
use rp_pico::hal::{
    clocks::init_clocks_and_plls,
    pac,
    pio::{PIOExt},
    sio::Sio,
    watchdog::Watchdog,
};
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

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    let program = pio_proc::pio_asm!(
        ".wrap_target",
        "out pins, 6    [1]",  // Output RGB data for both rows
        "set pins, 1    [1]",  // Set CLK high
        "set pins, 0    [1]",  // Set CLK low
        ".wrap"
    );

    let installed = pio.install(&program.program).unwrap();
    let (mut sm, _, mut tx) = rp2040_hal::pio::PIOBuilder::from_program(installed)
        .out_pins(2, 6)
        .set_pins(11, 1)
        .clock_divisor_fixed_point(1, 0)
        .build(sm0);

    sm.set_pindirs([
        (2, rp2040_hal::pio::PinDir::Output),
        (3, rp2040_hal::pio::PinDir::Output),
        (4, rp2040_hal::pio::PinDir::Output),
        (5, rp2040_hal::pio::PinDir::Output),
        (8, rp2040_hal::pio::PinDir::Output),
        (9, rp2040_hal::pio::PinDir::Output),
        (11, rp2040_hal::pio::PinDir::Output),
    ]);

    let dma = pac.DMA.split(&mut pac.RESETS);
    let dma_chan = dma.ch0;

    let mut framebuffer = [[0u32; WIDTH]; ROWS];

    // Configure address pins
    let mut a = pins.gpio10.into_push_pull_output();
    let mut b = pins.gpio16.into_push_pull_output();
    let mut c = pins.gpio18.into_push_pull_output();
    let mut d = pins.gpio20.into_push_pull_output();
    let mut e = pins.gpio22.into_push_pull_output();

    // Configure control pins
    let mut stb = pins.gpio12.into_push_pull_output();
    let mut oe = pins.gpio13.into_push_pull_output();

    sm.start();

    loop {
        for row in 0..ROWS {
            // rprintln!("Row {}",row);
            // Set row address
            a.set_state((row & 1 != 0).into()).unwrap();
            b.set_state(((row >> 1) & 1 != 0).into()).unwrap();
            c.set_state(((row >> 2) & 1 != 0).into()).unwrap();
            d.set_state(((row >> 3) & 1 != 0).into()).unwrap();
            e.set_state(((row >> 4) & 1 != 0).into()).unwrap();

            // Start DMA transfer
            // let _ = dma_chan.write_buffer(&framebuffer[row], &mut tx);
            tx.write(u32::MAX);
            // tx.write(u32::MAX);
          

            // Wait for DMA transfer to complete
            // while dma_chan.is_busy() {}

            // Latch data
            stb.set_high().unwrap();
            stb.set_low().unwrap();

            // Enable output
            oe.set_low().unwrap();

            // Delay for PWM (adjust as needed)
            cortex_m::asm::delay(1000);

            // Disable output
            oe.set_high().unwrap();
        }
    }
}
