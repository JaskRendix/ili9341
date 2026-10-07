//! ILI9341 Command definitions.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    SoftwareReset = 0x01,
    MemoryAccessControl = 0x36,
    PixelFormatSet = 0x3a,
    SleepModeOn = 0x10,
    SleepModeOff = 0x11,
    InvertOff = 0x20,
    InvertOn = 0x21,
    DisplayOff = 0x28,
    DisplayOn = 0x29,
    ColumnAddressSet = 0x2a,
    PageAddressSet = 0x2b,
    MemoryWrite = 0x2c,
    VerticalScrollDefine = 0x33,
    VerticalScrollAddr = 0x37,
    IdleModeOff = 0x38,
    IdleModeOn = 0x39,
    SetBrightness = 0x51,
    ContentAdaptiveBrightness = 0x55,
    NormalModeFrameRate = 0xb1,
    IdleModeFrameRate = 0xb2,
}
