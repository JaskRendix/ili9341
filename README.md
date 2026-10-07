# `ili9341-driver`

A platform-agnostic, `no_std` Rust driver for ILI9341 and ILI9340C TFT LCD displays.

This driver is designed for embedded Rust projects and aims to provide a simple, flexible interface for SPI-based TFT displays while remaining compatible with the wider embedded Rust ecosystem.

## Features

- `no_std` compatible
- Platform-agnostic design
- SPI-based display communication
- Support for ILI9341 and ILI9340C displays
- Display orientation and rotation
- Query current display orientation
- Hardware scrolling
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

- [ESP32-C6](examples/esp32c6/main.rs)
- [Raspberry Pi Pico](examples/rppico/main.rs)

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

## Project Status

### Planned improvements

- Support for additional resolutions
- Display memory reads
- DMA-friendly APIs
- More board examples

## Relationship to the Original Project

This crate is a modernized and independently maintained continuation of
https://github.com/yuri91/ili9341-rs.

The original project provided the foundation for this driver. This fork
focuses on updated dependencies, improved documentation, modern embedded
Rust support, and ongoing maintenance.

All original authors and contributors remain credited through the Git
history and license files.

## Contributing

Contributions, issue reports, testing, and hardware feedback are welcome.

Before opening a pull request, run:

```bash
cargo fmt
cargo check
cargo test
cargo test --doc
```

If you are planning a larger API or architectural change, open an issue first so the approach can be discussed.

Useful contributions include:

- Testing with different display modules
- Adding support for new boards
- Improving examples
- Writing documentation
- Adding tests
- Improving error messages
- Updating `embedded-hal` compatibility
- Reporting hardware-specific issues

## License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

You may choose either license.

## Acknowledgements

This project is derived from and builds upon the original [`yuri91/ili9341-rs`](https://github.com/yuri91/ili9341-rs) project.

Many thanks to the original author and all previous contributors who helped establish the driver and its embedded Rust support.
