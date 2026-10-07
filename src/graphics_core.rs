use crate::Ili9341;
use embedded_graphics_core::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};

impl<IFACE, RESET> OriginDimensions for Ili9341<IFACE, RESET> {
    fn size(&self) -> Size {
        Size::new(self.width() as u32, self.height() as u32)
    }
}

impl<IFACE, RESET> DrawTarget for Ili9341<IFACE, RESET>
where
    IFACE: display_interface::WriteOnlyDataCommand,
{
    type Error = display_interface::DisplayError;
    type Color = Rgb565;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bounding_box = self.bounding_box();

        for Pixel(point, color) in pixels {
            if bounding_box.contains(point) {
                let x = point.x as u16;
                let y = point.y as u16;
                let raw_color = color.into_storage();
                self.draw_raw_slice(x, y, x, y, &[raw_color])?;
            }
        }
        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let bounding_box = self.bounding_box();
        let drawable_area = area.intersection(&bounding_box);

        if let Some(drawable_bottom_right) = drawable_area.bottom_right() {
            let x0 = drawable_area.top_left.x as u16;
            let y0 = drawable_area.top_left.y as u16;
            let x1 = drawable_bottom_right.x as u16;
            let y1 = drawable_bottom_right.y as u16;

            if area == &drawable_area {
                // All pixels are on screen (Fast Path)
                self.draw_raw_iter(
                    x0,
                    y0,
                    x1,
                    y1,
                    area.points()
                        .zip(colors)
                        .map(|(_, color)| color.into_storage()),
                )
            } else {
                // Some pixels are clipped/off-screen
                self.draw_raw_iter(
                    x0,
                    y0,
                    x1,
                    y1,
                    area.points()
                        .zip(colors)
                        .filter(|(point, _)| drawable_area.contains(*point))
                        .map(|(_, color)| color.into_storage()),
                )
            }
        } else {
            // No pixels are on screen
            Ok(())
        }
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.clear_screen(color.into_storage())
    }
}
