#![no_std]

//! ILI9341 Display Driver
//!
//! ### Usage
//!
//! To control the display you need to set up:
//!
//! * Interface for communicating with display ([display-interface-spi crate] for SPI)
//! * Configuration (reset pin, delay, orientation and size) for display
//!
//! ```ignore
//! let iface = SPIInterface::new(spi, dc, cs);
//!
//! let mut display = Ili9341::new(
//!     iface,
//!     reset\_gpio,
//!     &mut delay,
//!     Orientation::Landscape,
//!     ili9341\_driver::DisplaySize240x320,
//! )
//! .unwrap();
//!
//! display.clear\_screen(Rgb565::RED).unwrap()
//! ```
//!
//! [display-interface-spi crate]: https://crates.io/crates/display-interface-spi

mod command;
mod modes;
mod scroller;

use command::Command;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use display_interface::DataFormat;
use display_interface::WriteOnlyDataCommand;

#[cfg(feature = "graphics")]
mod graphics_core;

pub use embedded_hal::spi::MODE_0 as SPI_MODE;

pub use display_interface::DisplayError;

pub use modes::{
    AdaptiveBrightness, DisplaySize, DisplaySize240x320, DisplaySize320x480, FrameRate,
    FrameRateClockDivision, Mode, ModeState, Orientation,
};
pub use scroller::Scroller;

type Result<T = (), E = DisplayError> = core::result::Result<T, E>;

/// There are two methods for drawing to the screen:
/// [Ili9341::draw_raw_iter] and [Ili9341::draw_raw_slice]
///
/// In both cases the expected pixel format is rgb565.
///
/// The hardware makes it efficient to draw rectangles on the screen.
///
/// What happens is the following:
///
/// - A drawing window is prepared (with the 2 opposite corner coordinates)
/// - The starting point for drawing is the top left corner of this window
/// - Every pair of bytes received is interpreted as a pixel value in rgb565
/// - As soon as a pixel is received, an internal counter is incremented,
///   and the next word will fill the next pixel (the adjacent on the right, or
///   the first of the next row if the row ended)
pub struct Ili9341<IFACE, RESET> {
    interface: IFACE,
    reset: RESET,
    width: usize,
    height: usize,
    landscape: bool,
}

impl<IFACE, RESET> Ili9341<IFACE, RESET>
where
    IFACE: WriteOnlyDataCommand,
    RESET: OutputPin,
{
    pub fn new<DELAY, SIZE, MODE>(
        interface: IFACE,
        reset: RESET,
        delay: &mut DELAY,
        mode: MODE,
        _display_size: SIZE,
    ) -> Result<Self>
    where
        DELAY: DelayNs,
        SIZE: DisplaySize,
        MODE: Mode,
    {
        let mut ili9341 = Ili9341 {
            interface,
            reset,
            width: SIZE::WIDTH,
            height: SIZE::HEIGHT,
            landscape: false,
        };

        // Do hardware reset by holding reset low for at least 10us
        ili9341.reset.set_low().map_err(|_| DisplayError::RSError)?;
        delay.delay_ms(1);

        // Set high for normal operation
        ili9341
            .reset
            .set_high()
            .map_err(|_| DisplayError::RSError)?;

        // Wait 5ms after reset before sending commands
        // and 120ms before sending Sleep Out
        delay.delay_ms(5);

        // Do software reset
        ili9341.command(Command::SoftwareReset, &[])?;

        // Wait 5ms after reset before sending commands
        // and 120ms before sending Sleep Out
        delay.delay_ms(120);

        ili9341.set_orientation(mode)?;

        // Set pixel format to 16 bits per pixel
        ili9341.command(Command::PixelFormatSet, &[0x55])?;

        ili9341.sleep_mode(ModeState::Off)?;

        // Wait 5ms after Sleep Out before sending commands
        delay.delay_ms(5);

        ili9341.display_mode(ModeState::On)?;

        Ok(ili9341)
    }
}

impl<IFACE, RESET> Ili9341<IFACE, RESET>
where
    IFACE: WriteOnlyDataCommand,
{
    fn command(&mut self, cmd: Command, args: &[u8]) -> Result {
        self.interface.send_commands(DataFormat::U8(&[cmd as u8]))?;
        self.interface.send_data(DataFormat::U8(args))
    }

    fn write_iter<I: IntoIterator<Item = u16>>(&mut self, data: I) -> Result {
        self.command(Command::MemoryWrite, &[])?;
        use DataFormat::U16BEIter;
        self.interface.send_data(U16BEIter(&mut data.into_iter()))
    }

    fn write_slice(&mut self, data: &[u16]) -> Result {
        self.command(Command::MemoryWrite, &[])?;
        self.interface.send_data(DataFormat::U16(data))
    }

    /// Validates a drawing window and returns its pixel count.
    ///
    /// This method does not send commands to the display.
    fn validate_window(&self, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<usize> {
        if x0 > x1 || y0 > y1 || x1 >= self.width as u16 || y1 >= self.height as u16 {
            return Err(DisplayError::InvalidFormatError);
        }

        let width = x1 as usize - x0 as usize + 1;
        let height = y1 as usize - y0 as usize + 1;

        width
            .checked_mul(height)
            .ok_or(DisplayError::InvalidFormatError)
    }

    fn set_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) -> Result {
        // Validate coordinates before issuing window commands.
        self.validate_window(x0, y0, x1, y1)?;

        self.command(
            Command::ColumnAddressSet,
            &[
                (x0 >> 8) as u8,
                (x0 & 0xff) as u8,
                (x1 >> 8) as u8,
                (x1 & 0xff) as u8,
            ],
        )?;
        self.command(
            Command::PageAddressSet,
            &[
                (y0 >> 8) as u8,
                (y0 & 0xff) as u8,
                (y1 >> 8) as u8,
                (y1 & 0xff) as u8,
            ],
        )
    }

    /// Configures the screen for hardware-accelerated vertical scrolling.
    pub fn configure_vertical_scroll(
        &mut self,
        fixed_top_lines: u16,
        fixed_bottom_lines: u16,
    ) -> Result<Scroller> {
        let height = self.height as u16;

        let total_fixed = fixed_top_lines
            .checked_add(fixed_bottom_lines)
            .ok_or(DisplayError::InvalidFormatError)?;

        if total_fixed > height {
            return Err(DisplayError::InvalidFormatError);
        }

        let scroll_lines = height - total_fixed;

        self.command(
            Command::VerticalScrollDefine,
            &[
                (fixed_top_lines >> 8) as u8,
                (fixed_top_lines & 0xff) as u8,
                (scroll_lines >> 8) as u8,
                (scroll_lines & 0xff) as u8,
                (fixed_bottom_lines >> 8) as u8,
                (fixed_bottom_lines & 0xff) as u8,
            ],
        )?;

        Ok(Scroller::new(fixed_top_lines, fixed_bottom_lines, height))
    }

    pub fn scroll_vertically(&mut self, scroller: &mut Scroller, num_lines: u16) -> Result {
        let scroll_region_height =
            scroller.height - scroller.fixed_top_lines - scroller.fixed_bottom_lines;

        if scroll_region_height > 0 {
            // Normalize the scroll lines using modulo arithmetic over the scroll region.
            let effective_lines = (num_lines % scroll_region_height) as u32;
            let current_relative = (scroller.top_offset - scroller.fixed_top_lines) as u32;

            let next_relative = (current_relative + effective_lines) % scroll_region_height as u32;
            scroller.top_offset = scroller.fixed_top_lines + next_relative as u16;
        }

        self.command(
            Command::VerticalScrollAddr,
            &[
                (scroller.top_offset >> 8) as u8,
                (scroller.top_offset & 0xff) as u8,
            ],
        )
    }

    /// Draw a rectangle on the screen, represented by top-left corner (x0, y0)
    /// and bottom-right corner (x1, y1).
    ///
    /// The border is included.
    ///
    /// This method accepts an iterator of rgb565 pixel values.
    ///
    /// The iterator is useful to avoid wasting memory by holding a buffer for
    /// the whole screen when it is not necessary.
    pub fn draw_raw_iter<I: IntoIterator<Item = u16>>(
        &mut self,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
        data: I,
    ) -> Result {
        self.set_window(x0, y0, x1, y1)?;
        self.write_iter(data)
    }

    /// Draw a rectangle on the screen, represented by top-left corner (x0, y0)
    /// and bottom-right corner (x1, y1).
    ///
    /// The border is included.
    ///
    /// This method accepts a raw buffer of words that will be copied to the screen
    /// video memory. The buffer must contain exactly one pixel for every pixel
    /// in the drawing window.
    ///
    /// The expected format is rgb565.
    pub fn draw_raw_slice(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, data: &[u16]) -> Result {
        // Validate the window and pixel count before sending any commands.
        let expected_pixels = self.validate_window(x0, y0, x1, y1)?;

        if data.len() != expected_pixels {
            return Err(DisplayError::InvalidFormatError);
        }

        self.set_window(x0, y0, x1, y1)?;
        self.write_slice(data)
    }

    /// Fill a rectangle with a single rgb565 color.
    ///
    /// `x` and `y` specify the top-left corner. `width` and `height` are
    /// measured in pixels.
    pub fn fill_rect(&mut self, x: u16, y: u16, width: u16, height: u16, color: u16) -> Result {
        if width == 0 || height == 0 {
            return Err(DisplayError::InvalidFormatError);
        }

        let x1 = x
            .checked_add(width - 1)
            .ok_or(DisplayError::InvalidFormatError)?;
        let y1 = y
            .checked_add(height - 1)
            .ok_or(DisplayError::InvalidFormatError)?;

        let pixel_count = self.validate_window(x, y, x1, y1)?;

        self.set_window(x, y, x1, y1)?;
        self.write_iter(core::iter::repeat_n(color, pixel_count))
    }

    /// Change the orientation of the screen.
    pub fn set_orientation<MODE>(&mut self, mode: MODE) -> Result
    where
        MODE: Mode,
    {
        self.command(Command::MemoryAccessControl, &[mode.mode()])?;

        if self.landscape ^ mode.is_landscape() {
            core::mem::swap(&mut self.height, &mut self.width);
        }

        self.landscape = mode.is_landscape();
        Ok(())
    }

    /// Fill the entire screen with the specified rgb565 color.
    pub fn clear_screen(&mut self, color: u16) -> Result {
        self.fill_rect(0, 0, self.width as u16, self.height as u16, color)
    }

    fn set_state_cmd(&mut self, mode: ModeState, on: Command, off: Command) -> Result {
        match mode {
            ModeState::On => self.command(on, &[]),
            ModeState::Off => self.command(off, &[]),
        }
    }

    /// Control the screen sleep mode.
    pub fn sleep_mode(&mut self, mode: ModeState) -> Result {
        self.set_state_cmd(mode, Command::SleepModeOn, Command::SleepModeOff)
    }

    /// Control the screen display mode.
    pub fn display_mode(&mut self, mode: ModeState) -> Result {
        self.set_state_cmd(mode, Command::DisplayOn, Command::DisplayOff)
    }

    /// Invert the pixel color on screen.
    pub fn invert_mode(&mut self, mode: ModeState) -> Result {
        self.set_state_cmd(mode, Command::InvertOn, Command::InvertOff)
    }

    /// Idle mode reduces the number of colors to 8.
    pub fn idle_mode(&mut self, mode: ModeState) -> Result {
        self.set_state_cmd(mode, Command::IdleModeOn, Command::IdleModeOff)
    }

    /// Set display brightness to a value between 0 and 255.
    pub fn brightness(&mut self, brightness: u8) -> Result {
        self.command(Command::SetBrightness, &[brightness])
    }

    /// Set adaptive brightness value equal to [AdaptiveBrightness].
    pub fn content_adaptive_brightness(&mut self, value: AdaptiveBrightness) -> Result {
        self.command(Command::ContentAdaptiveBrightness, &[value as _])
    }

    /// Configure [FrameRateClockDivision] and [FrameRate] in normal mode.
    pub fn normal_mode_frame_rate(
        &mut self,
        clk_div: FrameRateClockDivision,
        frame_rate: FrameRate,
    ) -> Result {
        self.command(
            Command::NormalModeFrameRate,
            &[clk_div as _, frame_rate as _],
        )
    }

    /// Configure [FrameRateClockDivision] and [FrameRate] in idle mode.
    pub fn idle_mode_frame_rate(
        &mut self,
        clk_div: FrameRateClockDivision,
        frame_rate: FrameRate,
    ) -> Result {
        self.command(Command::IdleModeFrameRate, &[clk_div as _, frame_rate as _])
    }
}

impl<IFACE, RESET> Ili9341<IFACE, RESET> {
    /// Get the current screen width. It can change based on the current orientation.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the current screen height. It can change based on the current orientation.
    pub fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod lib_test;

#[cfg(test)]
#[cfg(feature = "graphics")]
mod graphics_core_test;
