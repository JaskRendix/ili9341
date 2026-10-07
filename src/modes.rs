//! Display configuration types, sizes, and modes.

/// Trait that defines display size information
pub trait DisplaySize {
    /// Width in pixels
    const WIDTH: usize;
    /// Height in pixels
    const HEIGHT: usize;
}

/// Generic display size of 240x320 pixels
pub struct DisplaySize240x320;

impl DisplaySize for DisplaySize240x320 {
    const WIDTH: usize = 240;
    const HEIGHT: usize = 320;
}

/// Generic display size of 320x480 pixels
pub struct DisplaySize320x480;

impl DisplaySize for DisplaySize320x480 {
    const WIDTH: usize = 320;
    const HEIGHT: usize = 480;
}

/// For quite a few boards (ESP32-S2-Kaluga-1, M5Stack, M5Core2 and others),
/// the ILI9341 initialization command arguments are slightly different
///
/// This trait provides the flexibility for users to define their own
/// initialization command arguments suitable for the particular board they are using
pub trait Mode {
    fn mode(&self) -> u8;

    fn is_landscape(&self) -> bool;
}

/// The default implementation of the Mode trait from above
/// Should work for most (but not all) boards
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    Portrait,
    PortraitFlipped,
    Landscape,
    LandscapeFlipped,
}

impl Mode for Orientation {
    fn mode(&self) -> u8 {
        match self {
            Self::Portrait => 0x40 | 0x08,
            Self::Landscape => 0x20 | 0x08,
            Self::PortraitFlipped => 0x80 | 0x08,
            Self::LandscapeFlipped => 0x40 | 0x80 | 0x20 | 0x08,
        }
    }

    fn is_landscape(&self) -> bool {
        match self {
            Self::Landscape | Self::LandscapeFlipped => true,
            Self::Portrait | Self::PortraitFlipped => false,
        }
    }
}

/// Specify state of specific mode of operation
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModeState {
    On,
    Off,
}

/// Available Adaptive Brightness values
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdaptiveBrightness {
    Off = 0x00,
    UserInterfaceImage = 0x01,
    StillPicture = 0x02,
    MovingImage = 0x03,
}

/// Available frame rate in Hz
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameRate {
    FrameRate119 = 0x10,
    FrameRate112 = 0x11,
    FrameRate106 = 0x12,
    FrameRate100 = 0x13,
    FrameRate95 = 0x14,
    FrameRate90 = 0x15,
    FrameRate86 = 0x16,
    FrameRate83 = 0x17,
    FrameRate79 = 0x18,
    FrameRate76 = 0x19,
    FrameRate73 = 0x1a,
    FrameRate70 = 0x1b,
    FrameRate68 = 0x1c,
    FrameRate65 = 0x1d,
    FrameRate63 = 0x1e,
    FrameRate61 = 0x1f,
}

/// Frame rate clock division
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameRateClockDivision {
    Fosc = 0x00,
    FoscDiv2 = 0x01,
    FoscDiv4 = 0x02,
    FoscDiv8 = 0x03,
}
