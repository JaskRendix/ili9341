use crate::{DisplaySize240x320, Ili9341, Mode, ModeState, Orientation};
use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
use embedded_hal::digital::{ErrorType, OutputPin, PinState};

struct MockInterface;

impl WriteOnlyDataCommand for MockInterface {
    fn send_commands(&mut self, _cmd: DataFormat<'_>) -> Result<(), DisplayError> {
        Ok(())
    }

    fn send_data(&mut self, _buf: DataFormat<'_>) -> Result<(), DisplayError> {
        Ok(())
    }
}

struct MockPin;

impl ErrorType for MockPin {
    type Error = core::convert::Infallible;
}

impl OutputPin for MockPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_state(&mut self, _state: PinState) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct MockDelay;

impl embedded_hal::delay::DelayNs for MockDelay {
    fn delay_ns(&mut self, _ns: u32) {}
}

struct CustomMode;

impl Mode for CustomMode {
    fn mode(&self) -> u8 {
        0
    }

    fn is_landscape(&self) -> bool {
        true
    }
}

#[test]
fn test_display_initialization_and_dimensions() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    );

    assert!(display.is_ok());

    let display = display.unwrap();

    assert_eq!(display.width(), 240);
    assert_eq!(display.height(), 320);
}

#[test]
fn test_landscape_initialization_and_dimensions() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Landscape,
        DisplaySize240x320,
    )
    .unwrap();

    assert_eq!(display.width(), 320);
    assert_eq!(display.height(), 240);
}

#[test]
fn test_orientation_change() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert_eq!(display.width(), 240);
    assert_eq!(display.height(), 320);

    display.set_orientation(Orientation::Landscape).unwrap();

    assert_eq!(display.width(), 320);
    assert_eq!(display.height(), 240);

    display.set_orientation(Orientation::Portrait).unwrap();

    assert_eq!(display.width(), 240);
    assert_eq!(display.height(), 320);
}

#[test]
fn test_repeated_orientation_changes_do_not_change_dimensions() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    for _ in 0..5 {
        display.set_orientation(Orientation::Landscape).unwrap();
        assert_eq!(display.width(), 320);
        assert_eq!(display.height(), 240);

        display.set_orientation(Orientation::Portrait).unwrap();
        assert_eq!(display.width(), 240);
        assert_eq!(display.height(), 320);
    }
}

#[test]
fn test_clear_screen_in_portrait_mode() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.clear_screen(0xF800).is_ok());
}

#[test]
fn test_clear_screen_in_landscape_mode() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Landscape,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.clear_screen(0x07E0).is_ok());
}

#[test]
fn test_draw_raw_slice_accepts_valid_rectangle() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    let pixels = [0xF800, 0x07E0, 0x001F, 0xFFFF];

    assert!(display.draw_raw_slice(0, 0, 1, 1, &pixels).is_ok());
}

#[test]
fn test_draw_raw_iter_accepts_iterator() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    let pixels = [0xF800, 0x07E0, 0x001F, 0xFFFF];

    assert!(
        display
            .draw_raw_iter(0, 0, 1, 1, pixels.into_iter())
            .is_ok()
    );
}

#[test]
fn test_single_pixel_draw() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(
        display
            .draw_raw_slice(100, 100, 100, 100, &[0xFFFF])
            .is_ok()
    );
}

#[test]
fn test_full_screen_draw() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    let pixels = core::iter::repeat_n(0x0000, 240 * 320);

    assert!(display.draw_raw_iter(0, 0, 239, 319, pixels).is_ok());
}

#[test]
fn test_invalid_drawing_windows_are_rejected() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    // x0 must not be greater than x1.
    assert!(display.draw_raw_slice(10, 10, 9, 20, &[]).is_err());

    // y0 must not be greater than y1.
    assert!(display.draw_raw_slice(10, 20, 20, 19, &[]).is_err());

    // x1 must be less than the display width.
    assert!(display.draw_raw_slice(0, 0, 240, 319, &[]).is_err());

    // y1 must be less than the display height.
    assert!(display.draw_raw_slice(0, 0, 239, 320, &[]).is_err());
}

#[test]
fn test_last_valid_pixel_is_accepted() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(
        display
            .draw_raw_slice(239, 319, 239, 319, &[0xFFFF])
            .is_ok()
    );
}

#[test]
fn test_landscape_window_uses_landscape_dimensions() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Landscape,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(
        display
            .draw_raw_slice(319, 239, 319, 239, &[0xFFFF])
            .is_ok()
    );

    // These coordinates would be valid in portrait mode but not landscape mode.
    assert!(
        display
            .draw_raw_slice(239, 319, 239, 319, &[0xFFFF])
            .is_err()
    );
}

#[test]
fn test_vertical_scrolling_configuration() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.configure_vertical_scroll(10, 20).is_ok());
}

#[test]
fn test_invalid_vertical_scrolling_configuration_is_rejected() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    // 200 + 121 is greater than the 320-pixel display height.
    assert!(display.configure_vertical_scroll(200, 121).is_err());
}

#[test]
fn test_vertical_scrolling_with_no_scroll_region_is_safe() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    let mut scroller = display.configure_vertical_scroll(160, 160).unwrap();

    assert!(display.scroll_vertically(&mut scroller, 100).is_ok());
}

#[test]
fn test_vertical_scrolling_wraps_without_error() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    let mut scroller = display.configure_vertical_scroll(10, 20).unwrap();

    // Scroll region height is 290 pixels. Scrolling by more than one
    // complete region must still succeed.
    assert!(display.scroll_vertically(&mut scroller, 290).is_ok());
    assert!(display.scroll_vertically(&mut scroller, 580).is_ok());
    assert!(display.scroll_vertically(&mut scroller, 1).is_ok());
}

#[test]
fn test_sleep_mode() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.sleep_mode(ModeState::On).is_ok());
    assert!(display.sleep_mode(ModeState::Off).is_ok());
}

#[test]
fn test_display_mode() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.display_mode(ModeState::Off).is_ok());
    assert!(display.display_mode(ModeState::On).is_ok());
}

#[test]
fn test_invert_mode() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.invert_mode(ModeState::On).is_ok());
    assert!(display.invert_mode(ModeState::Off).is_ok());
}

#[test]
fn test_idle_mode() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.idle_mode(ModeState::On).is_ok());
    assert!(display.idle_mode(ModeState::Off).is_ok());
}

#[test]
fn test_brightness_boundaries() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.brightness(0).is_ok());
    assert!(display.brightness(128).is_ok());
    assert!(display.brightness(255).is_ok());
}

#[test]
fn test_draw_raw_slice_rejects_too_few_pixels() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    // A 2 × 2 window requires 4 pixels.
    assert!(display.draw_raw_slice(0, 0, 1, 1, &[0xFFFF]).is_err());
}

#[test]
fn test_draw_raw_slice_rejects_too_many_pixels() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    // A 2 × 2 window requires exactly 4 pixels.
    let pixels = [0xFFFF; 5];
    assert!(display.draw_raw_slice(0, 0, 1, 1, &pixels).is_err());
}

#[test]
fn test_fill_rect_accepts_valid_rectangle() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.fill_rect(10, 20, 3, 4, 0xF800).is_ok());
}

#[test]
fn test_fill_rect_accepts_single_pixel() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.fill_rect(239, 319, 1, 1, 0xFFFF).is_ok());
}

#[test]
fn test_fill_rect_rejects_zero_dimensions() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.fill_rect(0, 0, 0, 1, 0xFFFF).is_err());
    assert!(display.fill_rect(0, 0, 1, 0, 0xFFFF).is_err());
}

#[test]
fn test_fill_rect_rejects_out_of_bounds_rectangle() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    // The rectangle would extend past the right edge.
    assert!(display.fill_rect(239, 0, 2, 1, 0xFFFF).is_err());

    // The rectangle would extend past the bottom edge.
    assert!(display.fill_rect(0, 319, 1, 2, 0xFFFF).is_err());
}

#[test]
fn test_fill_rect_rejects_endpoint_overflow() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.fill_rect(u16::MAX, 0, 2, 1, 0xFFFF).is_err());
    assert!(display.fill_rect(0, u16::MAX, 1, 2, 0xFFFF).is_err());
}

#[test]
fn test_fill_rect_accepts_maximum_valid_rectangle() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert!(display.fill_rect(0, 0, 240, 320, 0xFFFF).is_ok());
}

#[test]
fn test_orientation_reports_portrait() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert_eq!(display.orientation(), Some(Orientation::Portrait));
}

#[test]
fn test_orientation_updates_after_change() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    assert_eq!(display.orientation(), Some(Orientation::Portrait));

    display.set_orientation(Orientation::Landscape).unwrap();

    assert_eq!(display.orientation(), Some(Orientation::Landscape));

    display
        .set_orientation(Orientation::PortraitFlipped)
        .unwrap();

    assert_eq!(display.orientation(), Some(Orientation::PortraitFlipped));
}

#[test]
fn test_flipped_orientation_is_preserved() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    display
        .set_orientation(Orientation::LandscapeFlipped)
        .unwrap();

    assert_eq!(display.orientation(), Some(Orientation::LandscapeFlipped));

    display
        .set_orientation(Orientation::PortraitFlipped)
        .unwrap();

    assert_eq!(display.orientation(), Some(Orientation::PortraitFlipped));
}

#[test]
fn test_custom_mode_orientation_is_none() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    display.set_orientation(CustomMode).unwrap();

    assert_eq!(display.orientation(), None);
}

#[test]
fn test_orientation_reports_landscape() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Landscape,
        DisplaySize240x320,
    )
    .unwrap();

    assert_eq!(display.orientation(), Some(Orientation::Landscape));
}

#[test]
fn test_flipped_orientations_keep_correct_dimensions() {
    let iface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    let mut display = Ili9341::new(
        iface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    display
        .set_orientation(Orientation::LandscapeFlipped)
        .unwrap();

    assert_eq!(display.width(), 320);
    assert_eq!(display.height(), 240);

    display
        .set_orientation(Orientation::PortraitFlipped)
        .unwrap();

    assert_eq!(display.width(), 240);
    assert_eq!(display.height(), 320);
}
