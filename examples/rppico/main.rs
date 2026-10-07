//! Raspberry Pi Pico (RP2040) ILI9341 example
//!
//! PIN ASSIGNMENTS
//!   GP10 (PIN14): DC
//!   GP11 (PIN15): RESET
//!   GP12 (PIN16): MISO
//!   GP13 (PIN17): CS
//!   GP14 (PIN19): SCK
//!   GP15 (PIN20): MOSI

#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use defmt::*;
use defmt_rtt as _;
use display_interface_spi::SPIInterface;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use ili9341_driver::{DisplayError, Ili9341};
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
    info!("Program start");

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

    // SPI1:
    //   GP15 = MOSI
    //   GP12 = MISO
    //   GP14 = SCK
    //
    // The explicit `8` selects 8-bit SPI words and resolves the
    // const-generic ambiguity in newer rp2040-hal versions.
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
        ili9341_driver::Orientation::Landscape,
        ili9341_driver::DisplaySize240x320,
    )
    .unwrap();

    draw(&mut display);

    loop {
        cortex_m::asm::nop();
    }
}

fn draw<T>(display: &mut T)
where
    T: DrawTarget<Color = Rgb565, Error = DisplayError>,
{
    let line_style = PrimitiveStyle::with_stroke(Rgb565::BLUE, 1);
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

    Circle::new(Point::new(72, 8), 48)
        .into_styled(line_style)
        .draw(display)
        .unwrap();

    Line::new(Point::new(48, 16), Point::new(8, 16))
        .into_styled(line_style)
        .draw(display)
        .unwrap();

    Line::new(Point::new(48, 16), Point::new(64, 32))
        .into_styled(line_style)
        .draw(display)
        .unwrap();

    Rectangle::new(Point::new(79, 15), Size::new(34, 34))
        .into_styled(line_style)
        .draw(display)
        .unwrap();

    Text::new("Hello World!", Point::new(5, 5), text_style)
        .draw(display)
        .unwrap();
}
