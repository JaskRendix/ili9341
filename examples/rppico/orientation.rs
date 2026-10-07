//! Raspberry Pi Pico (RP2040) orientation demonstration.
//!
//! Cycles through all supported display orientations every two seconds.

#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use defmt::*;
use defmt_rtt as _;
use display_interface_spi::SPIInterface;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, Text},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use ili9341_driver::{DisplayError, DisplaySize240x320, Ili9341, Orientation};
use panic_halt as _;
use rp_pico::{
    entry,
    hal::{
        Spi,
        clocks::{Clock, init_clocks_and_plls},
        fugit::RateExtU32,
        gpio::FunctionSpi,
        pac,
        sio::Sio,
        watchdog::Watchdog,
    },
};

#[entry]
fn main() -> ! {
    info!("Orientation example");

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // The Raspberry Pi Pico uses a 12 MHz external crystal.
    let external_xtal_freq_hz = 12_000_000u32;

    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let spi = Spi::<_, _, _, 8>::new(
        pac.SPI1,
        (
            pins.gpio15.into_function::<FunctionSpi>(),
            pins.gpio12.into_function::<FunctionSpi>(),
            pins.gpio14.into_function::<FunctionSpi>(),
        ),
    )
    .init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16_000_000u32.Hz(),
        &embedded_hal::spi::MODE_0,
    );

    let cs = pins.gpio13.into_push_pull_output();

    let spi_device = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let dc = pins.gpio10.into_push_pull_output();

    let spi_interface = SPIInterface::new(spi_device, dc);

    let reset = pins.gpio11.into_push_pull_output();

    let mut display = Ili9341::new(
        spi_interface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    let orientations = [
        Orientation::Portrait,
        Orientation::Landscape,
        Orientation::PortraitFlipped,
        Orientation::LandscapeFlipped,
    ];

    loop {
        for orientation in orientations {
            display.set_orientation(orientation).unwrap();

            display.clear_screen(Rgb565::BLACK.into_storage()).unwrap();

            let width = display.width() as i32;
            let height = display.height() as i32;

            draw_orientation(&mut display, orientation, width, height);

            delay.delay_ms(2000);
        }
    }
}

fn draw_orientation<T>(display: &mut T, orientation: Orientation, width: i32, height: i32)
where
    T: DrawTarget<Color = Rgb565, Error = DisplayError>,
{
    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);

    let text = match orientation {
        Orientation::Portrait => "Portrait",
        Orientation::Landscape => "Landscape",
        Orientation::PortraitFlipped => "Portrait Flipped",
        Orientation::LandscapeFlipped => "Landscape Flipped",
    };

    Text::with_alignment(
        text,
        Point::new(width / 2, height / 2),
        style,
        Alignment::Center,
    )
    .draw(display)
    .unwrap();
}
