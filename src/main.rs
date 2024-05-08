#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use embedded_alloc::Heap;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use embedded_hal::blocking::delay::DelayMs;
use embedded_mogeefont::MogeeTextStyle;
use fugit::RateExtU32;
use panic_halt as _; // exit the program if a panic occurs
use rp_pico::entry;
use rp_pico::hal;
use sh1106::{prelude::*, Builder};

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn init_heap() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

#[entry]
fn does_not_have_to_be_main() -> ! {
    init_heap();
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
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

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure two pins as being I²C, not GPIO
    let sda_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, hal::gpio::PullUp> =
        pins.gpio0.reconfigure();
    let scl_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, hal::gpio::PullUp> =
        pins.gpio1.reconfigure();

    // Create the I²C driver, using the two pre-configured pins
    let i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
    display.init().unwrap();

    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let mut count = 0;

    loop {
        display.clear();

        count += 1;

        Text::with_baseline(
            format!(
                "There is so much screen\nspace for MogeeFont! Enough\nto fit a haiku. Should I make\nit scroll through a book?\nDynamic counter: {}",
                count
            )
            .as_str(),
            Point::new(10, 5),
            MogeeTextStyle::new(BinaryColor::On),
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();

        display.flush().unwrap();

        timer.delay_ms(1000);
    }
}
