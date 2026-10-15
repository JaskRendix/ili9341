# `ili9341-driver`

A platform-agnostic, `no_std` Rust driver for ILI9341 and ILI9340C TFT LCD displays.

This driver is designed for embedded Rust projects and aims to provide a simple, flexible interface for SPI-based TFT displays while remaining compatible with the wider embedded Rust ecosystem.

## Features

- `no_std` compatible
- Platform-agnostic design
- SPI-based display communication
- Support for ILI9341 and ILI9340C displays
- Display orientation control
- Query current display orientation
- Hardware vertical scrolling
- Partial display mode
- Partial display area configuration
- Gamma curve selection
- Frame-rate configuration
  - Normal mode
  - Idle mode
  - Partial mode
- Display brightness control
- Content Adaptive Brightness Control (CABC)
- Pixel drawing
- Rectangle fills
- [`embedded-graphics`] integration
- Suitable for microcontrollers and embedded Linux platforms

[`embedded-graphics`]: https://docs.rs/embedded-graphics

## Supported Displays

This driver is designed for TFT displays based on:

- ILI9341
- ILI9340C

Display modules based on the same controller may differ in resolution, wiring, touch controller, and pin layout. Check your module's documentation before connecting it to your hardware.

## Examples

Board-specific examples are available in the [`examples`](examples) directory.

Currently available examples include:

- ESP32-C6
  - `examples/esp32c6/main.rs`
  - `examples/esp32c6/orientation.rs`

- Raspberry Pi Pico
  - `examples/rppico/main.rs`
  - `examples/rppico/orientation.rs`

- STM32F411 BlackPill (RTIC)
  - `examples/rtic/main.rs`

A typical display setup requires:

- SPI
- Data/command pin
- Chip-select pin
- Reset pin
- A compatible delay provider
- A compatible `embedded-hal` implementation

The exact initialization code depends on the board and HAL being used.

## Installation

This crate is currently not published on crates.io.

To use the latest version directly from GitHub:

```toml
[dependencies]
ili9341-driver = { git = "https://github.com/JaskRendix/ili9341" }
```

## Usage

The driver is intended to work with embedded Rust display interfaces and [`embedded-graphics`]. A simplified example looks like this:

```rust
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

let mut display = /* initialize the ILI9341 display */;

Rectangle::new(Point::new(0, 0), Size::new(100, 100))
    .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
    .draw(&mut display)?;
```

Refer to the board examples and API documentation for complete initialization code.

## API Highlights

- Display orientation control
- Hardware vertical scrolling
- Partial display mode
- Gamma curve selection
- Frame-rate configuration
- Brightness and CABC control

## Project Status

### Planned improvements

- Support for additional resolutions
- Display memory reads
- Tearing effect (TE) support
- DMA-friendly APIs
- Additional board and feature examples

## Relationship to the Original Project

This crate is a modernized and independently maintained continuation of
https://github.com/yuri91/ili9341-rs.

The original project provided the foundation for this driver. This fork
focuses on updated dependencies, improved documentation, modern embedded
Rust support, and ongoing maintenance.

All original authors and contributors remain credited through the Git
history and license files.

## License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

You may choose either license.

## Acknowledgements

This project is derived from and builds upon the original [`yuri91/ili9341-rs`](https://github.com/yuri91/ili9341-rs) project.

Many thanks to the original author and all previous contributors who helped establish the driver and its embedded Rust support.

## Documentation

* ILI9341 Datasheet (Adafruit/Ilitek): [https://cdn-shop.adafruit.com/datasheets/ILI9341.pdf](https://cdn-shop.adafruit.com/datasheets/ILI9341.pdf)
* ILI9341 Controller Summary (Crystalfontz): [https://www.crystalfontz.com/controllers/Ilitek/ILI9341](https://www.crystalfontz.com/controllers/Ilitek/ILI9341)
