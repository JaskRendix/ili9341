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

    fn orientation(&self) -> Option<Orientation> {
        None
    }
}

// Memory Access Control (MADCTL, 36h) bit definitions
const MADCTL_MY: u8 = 1 << 7;
const MADCTL_MX: u8 = 1 << 6;
const MADCTL_MV: u8 = 1 << 5;
const MADCTL_BGR: u8 = 1 << 3;

/// The default implementation of the Mode trait from above
/// Should work for most (but not all) boards
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    Portrait,
    PortraitFlipped,
    Landscape,
    LandscapeFlipped,
}

impl Orientation {
    pub(crate) fn is_landscape(self) -> bool {
        matches!(self, Self::Landscape | Self::LandscapeFlipped)
    }
}

impl Mode for Orientation {
    fn mode(&self) -> u8 {
        match self {
            Self::Portrait => MADCTL_MX | MADCTL_BGR,

            Self::Landscape => MADCTL_MV | MADCTL_BGR,

            Self::PortraitFlipped => MADCTL_MY | MADCTL_BGR,

            Self::LandscapeFlipped => MADCTL_MX | MADCTL_MY | MADCTL_MV | MADCTL_BGR,
        }
    }

    fn is_landscape(&self) -> bool {
        (*self).is_landscape()
    }

    fn orientation(&self) -> Option<Orientation> {
        Some(*self)
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

/// Available gamma curves.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GammaCurve {
    Curve1 = 0x01,
    Curve2 = 0x02,
    Curve3 = 0x04,
    Curve4 = 0x08,
}

/// Available frame rates supported by the ILI9341.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameRate {
    Hz119 = 0x10,
    Hz112 = 0x11,
    Hz106 = 0x12,
    Hz100 = 0x13,
    Hz95 = 0x14,
    Hz90 = 0x15,
    Hz86 = 0x16,
    Hz83 = 0x17,
    Hz79 = 0x18,
    Hz76 = 0x19,
    Hz73 = 0x1a,
    Hz70 = 0x1b,
    Hz68 = 0x1c,
    Hz65 = 0x1d,
    Hz63 = 0x1e,
    Hz61 = 0x1f,
}

/// Internal oscillator division ratio used by frame rate control.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameRateClockDivision {
    Div1 = 0x00,
    Div2 = 0x01,
    Div4 = 0x02,
    Div8 = 0x03,
}
