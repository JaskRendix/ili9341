use crate::{DisplaySize240x320, Ili9341, Orientation};
use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
use embedded_graphics_core::{
    draw_target::DrawTarget, geometry::OriginDimensions, pixelcolor::Rgb565, prelude::*,
    primitives::Rectangle,
};
use embedded_hal::digital::{ErrorType, OutputPin, PinState};

struct MockInterface;

impl WriteOnlyDataCommand for MockInterface {
    fn send_commands(&mut self, _cmd: DataFormat<'_>) -> Result<(), DisplayError> {
        Ok(())
    }

    fn send_data(&mut self, _data: DataFormat<'_>) -> Result<(), DisplayError> {
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

fn create_display() -> Ili9341<MockInterface, MockPin> {
    let interface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    Ili9341::new(
        interface,
        reset,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap()
}

fn create_landscape_display() -> Ili9341<MockInterface, MockPin> {
    let interface = MockInterface;
    let reset = MockPin;
    let mut delay = MockDelay;

    Ili9341::new(
        interface,
        reset,
        &mut delay,
        Orientation::Landscape,
        DisplaySize240x320,
    )
    .unwrap()
}

#[test]
fn test_origin_dimensions_in_portrait_mode() {
    let display = create_display();

    assert_eq!(display.size(), Size::new(240, 320));
    assert_eq!(
        display.bounding_box(),
        Rectangle::new(Point::zero(), Size::new(240, 320))
    );
}

#[test]
fn test_origin_dimensions_in_landscape_mode() {
    let display = create_landscape_display();

    assert_eq!(display.size(), Size::new(320, 240));
    assert_eq!(
        display.bounding_box(),
        Rectangle::new(Point::zero(), Size::new(320, 240))
    );
}

#[test]
fn test_draw_iter_draws_an_in_bounds_pixel() {
    let mut display = create_display();

    let pixels = [Pixel(Point::new(10, 20), Rgb565::RED)];

    assert!(display.draw_iter(pixels).is_ok());
}

#[test]
fn test_draw_iter_draws_multiple_in_bounds_pixels() {
    let mut display = create_display();

    let pixels = [
        Pixel(Point::new(0, 0), Rgb565::RED),
        Pixel(Point::new(1, 0), Rgb565::GREEN),
        Pixel(Point::new(2, 0), Rgb565::BLUE),
        Pixel(Point::new(239, 319), Rgb565::WHITE),
    ];

    assert!(display.draw_iter(pixels).is_ok());
}

#[test]
fn test_draw_iter_ignores_pixels_above_display() {
    let mut display = create_display();

    let pixels = [
        Pixel(Point::new(-1, 0), Rgb565::RED),
        Pixel(Point::new(0, -1), Rgb565::GREEN),
        Pixel(Point::new(-100, -100), Rgb565::BLUE),
    ];

    assert!(display.draw_iter(pixels).is_ok());
}

#[test]
fn test_draw_iter_ignores_pixels_below_display() {
    let mut display = create_display();

    let pixels = [
        Pixel(Point::new(240, 0), Rgb565::RED),
        Pixel(Point::new(0, 320), Rgb565::GREEN),
        Pixel(Point::new(1000, 1000), Rgb565::BLUE),
    ];

    assert!(display.draw_iter(pixels).is_ok());
}

#[test]
fn test_draw_iter_accepts_an_empty_iterator() {
    let mut display = create_display();

    let pixels = core::iter::empty::<Pixel<Rgb565>>();

    assert!(display.draw_iter(pixels).is_ok());
}

#[test]
fn test_draw_iter_handles_mixed_in_bounds_and_out_of_bounds_pixels() {
    let mut display = create_display();

    let pixels = [
        Pixel(Point::new(-1, 0), Rgb565::RED),
        Pixel(Point::new(0, 0), Rgb565::GREEN),
        Pixel(Point::new(239, 319), Rgb565::BLUE),
        Pixel(Point::new(240, 319), Rgb565::WHITE),
    ];

    assert!(display.draw_iter(pixels).is_ok());
}

#[test]
fn test_fill_contiguous_fills_an_in_bounds_rectangle() {
    let mut display = create_display();

    let area = Rectangle::new(Point::new(10, 20), Size::new(4, 3));
    let colors = core::iter::repeat_n(Rgb565::RED, 12);

    assert!(display.fill_contiguous(&area, colors).is_ok());
}

#[test]
fn test_fill_contiguous_accepts_exactly_sized_color_iterator() {
    let mut display = create_display();

    let area = Rectangle::new(Point::new(0, 0), Size::new(2, 2));
    let colors = [Rgb565::RED, Rgb565::GREEN, Rgb565::BLUE, Rgb565::WHITE];

    assert!(display.fill_contiguous(&area, colors).is_ok());
}

#[test]
fn test_fill_contiguous_clips_area_at_top_left() {
    let mut display = create_display();

    let area = Rectangle::new(Point::new(-10, -10), Size::new(20, 20));
    let colors = core::iter::repeat_n(Rgb565::RED, 400);

    assert!(display.fill_contiguous(&area, colors).is_ok());
}

#[test]
fn test_fill_contiguous_clips_area_at_bottom_right() {
    let mut display = create_display();

    let area = Rectangle::new(Point::new(230, 310), Size::new(20, 20));
    let colors = core::iter::repeat_n(Rgb565::BLUE, 400);

    assert!(display.fill_contiguous(&area, colors).is_ok());
}

#[test]
fn test_fill_contiguous_clips_area_crossing_all_four_edges() {
    let mut display = create_display();

    let area = Rectangle::new(Point::new(-10, -10), Size::new(260, 340));
    let colors = core::iter::repeat_n(Rgb565::GREEN, 260 * 340);

    assert!(display.fill_contiguous(&area, colors).is_ok());
}

#[test]
fn test_fill_contiguous_handles_area_above_display() {
    let mut display = create_display();

    let area = Rectangle::new(Point::new(0, -100), Size::new(20, 20));
    let colors = core::iter::repeat_n(Rgb565::RED, 400);

    assert!(display.fill_contiguous(&area, colors).is_ok());
}

#[test]
fn test_fill_contiguous_handles_area_below_display() {
    let mut display = create_display();

    let area = Rectangle::new(Point::new(0, 400), Size::new(20, 20));
    let colors = core::iter::repeat_n(Rgb565::RED, 400);

    assert!(display.fill_contiguous(&area, colors).is_ok());
}

#[test]
fn test_fill_contiguous_handles_area_left_of_display() {
    let mut display = create_display();

    let area = Rectangle::new(Point::new(-100, 20), Size::new(20, 20));
    let colors = core::iter::repeat_n(Rgb565::RED, 400);

    assert!(display.fill_contiguous(&area, colors).is_ok());
}

#[test]
fn test_fill_contiguous_handles_area_right_of_display() {
    let mut display = create_display();

    let area = Rectangle::new(Point::new(300, 20), Size::new(20, 20));
    let colors = core::iter::repeat_n(Rgb565::RED, 400);

    assert!(display.fill_contiguous(&area, colors).is_ok());
}

#[test]
fn test_fill_contiguous_handles_zero_sized_area() {
    let mut display = create_display();

    let area = Rectangle::new(Point::new(20, 20), Size::zero());
    let colors = core::iter::empty::<Rgb565>();

    assert!(display.fill_contiguous(&area, colors).is_ok());
}

#[test]
fn test_fill_contiguous_handles_area_touching_display_edge() {
    let mut display = create_display();

    let area = Rectangle::new(Point::new(239, 319), Size::new(1, 1));
    let colors = core::iter::once(Rgb565::WHITE);

    assert!(display.fill_contiguous(&area, colors).is_ok());
}

#[test]
fn test_fill_contiguous_works_in_landscape_mode() {
    let mut display = create_landscape_display();

    let area = Rectangle::new(Point::new(10, 20), Size::new(20, 10));
    let colors = core::iter::repeat_n(Rgb565::RED, 200);

    assert!(display.fill_contiguous(&area, colors).is_ok());
}

#[test]
fn test_clear_uses_rgb565_color() {
    let mut display = create_display();

    assert!(display.clear(Rgb565::BLACK).is_ok());
    assert!(display.clear(Rgb565::RED).is_ok());
    assert!(display.clear(Rgb565::GREEN).is_ok());
    assert!(display.clear(Rgb565::BLUE).is_ok());
    assert!(display.clear(Rgb565::WHITE).is_ok());
}

#[test]
fn test_clear_works_in_landscape_mode() {
    let mut display = create_landscape_display();

    assert!(display.clear(Rgb565::BLACK).is_ok());
}
