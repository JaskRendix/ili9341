//! ILI9341 Screen Scroller.

/// Scroller must be provided in order to scroll the screen. It can only be obtained
/// by configuring the screen for scrolling.
pub struct Scroller {
    pub(crate) top_offset: u16,
    pub(crate) fixed_bottom_lines: u16,
    pub(crate) fixed_top_lines: u16,
    pub(crate) height: u16,
}

impl Scroller {
    pub(crate) fn new(fixed_top_lines: u16, fixed_bottom_lines: u16, height: u16) -> Scroller {
        Scroller {
            top_offset: fixed_top_lines,
            fixed_top_lines,
            fixed_bottom_lines,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Scroller;

    #[test]
    fn test_new_initializes_all_fields() {
        let scroller = Scroller::new(10, 20, 320);

        assert_eq!(scroller.top_offset, 10);
        assert_eq!(scroller.fixed_top_lines, 10);
        assert_eq!(scroller.fixed_bottom_lines, 20);
        assert_eq!(scroller.height, 320);
    }

    #[test]
    fn test_new_with_zero_fixed_regions() {
        let scroller = Scroller::new(0, 0, 320);

        assert_eq!(scroller.top_offset, 0);
        assert_eq!(scroller.fixed_top_lines, 0);
        assert_eq!(scroller.fixed_bottom_lines, 0);
        assert_eq!(scroller.height, 320);
    }

    #[test]
    fn test_new_with_entire_screen_fixed() {
        let scroller = Scroller::new(320, 0, 320);

        assert_eq!(scroller.top_offset, 320);
        assert_eq!(scroller.fixed_top_lines, 320);
        assert_eq!(scroller.fixed_bottom_lines, 0);
        assert_eq!(scroller.height, 320);
    }
}
