//! ESP32-C6 orientation demonstration.
//!
//! Cycles through all supported display orientations every two seconds.

#![no_std]
#![no_main]

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_8X13},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, Text},
};
use embedded_hal::delay::DelayNs;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Level, Output},
    main,
    peripherals::Peripherals,
    spi::{
        Mode,
        master::{Config, Spi},
    },
    time::RateExtU32,
};
use ili9341_driver::{DisplaySize240x320, Ili9341, Orientation};
use panic_halt as _;

#[main]
fn main() -> ! {
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals: Peripherals = esp_hal::init(config);

    let dc = peripherals.GPIO9;
    let mosi = peripherals.GPIO18;
    let sclk = peripherals.GPIO19;
    let miso = peripherals.GPIO20;
    let cs = peripherals.GPIO21;
    let rst = peripherals.GPIO22;

    let rst_output = Output::new(rst, Level::Low);
    let dc_output = Output::new(dc, Level::Low);

    let spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(100.kHz())
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sclk)
    .with_miso(miso)
    .with_mosi(mosi);

    let cs_output = Output::new(cs, Level::High);
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs_output).unwrap();
    let interface = SPIInterface::new(spi_device, dc_output);

    let mut delay = Delay::new();

    let mut display = Ili9341::new(
        interface,
        rst_output,
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

            let background = match orientation {
                Orientation::Portrait => Rgb565::RED,
                Orientation::Landscape => Rgb565::GREEN,
                Orientation::PortraitFlipped => Rgb565::BLUE,
                Orientation::LandscapeFlipped => Rgb565::YELLOW,
            };

            display.clear_screen(background.into_storage()).unwrap();

            let text = match orientation {
                Orientation::Portrait => "Portrait",
                Orientation::Landscape => "Landscape",
                Orientation::PortraitFlipped => "Portrait Flipped",
                Orientation::LandscapeFlipped => "Landscape Flipped",
            };

            let style = MonoTextStyle::new(&FONT_8X13, Rgb565::WHITE);

            Text::with_alignment(
                text,
                Point::new(display.width() as i32 / 2, display.height() as i32 / 2),
                style,
                Alignment::Center,
            )
            .draw(&mut display)
            .unwrap();

            delay.delay_ms(2000);
        }
    }
}
