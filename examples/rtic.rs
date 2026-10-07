//! Cortex-M RTIC example.
//!
//! Tested on a BlackPill development board using an STM32F411CEU
//! microcontroller.
//!
//! The LCD reset pin is hard-wired to VCC, so a dummy output pin is used
//! as the reset pin.

#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac)]
mod app {
    use core::convert::Infallible;

    use display_interface_spi::SPIInterface;
    use embedded_graphics::{
        mono_font::{MonoTextStyle, ascii::FONT_6X10},
        pixelcolor::Rgb565,
        prelude::*,
        text::{Alignment, Text},
    };
    use embedded_hal::digital::{ErrorType, OutputPin};
    use embedded_hal_bus::spi::ExclusiveDevice;
    use ili9341_driver::{DisplaySize240x320, Ili9341, Orientation};
    use stm32f4xx_hal::{
        prelude::*,
        rcc::Config,
        spi::{Mode, Phase, Polarity},
    };

    #[derive(Default)]
    pub struct DummyOutputPin;

    impl ErrorType for DummyOutputPin {
        type Error = Infallible;
    }

    impl OutputPin for DummyOutputPin {
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let dp = ctx.device;

        let mut rcc = dp.RCC.freeze(Config::hse(25.MHz()).sysclk(100.MHz()));

        let gpioa = dp.GPIOA.split(&mut rcc);
        let gpiob = dp.GPIOB.split(&mut rcc);

        let lcd_clk = gpiob.pb0.into_alternate();

        let lcd_mosi = gpioa.pa10.into_alternate().internal_pull_up(true);

        let lcd_dc = gpiob.pb1.into_push_pull_output();
        let lcd_cs = gpiob.pb2.into_push_pull_output();

        let mode = Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        };

        let lcd_spi = dp.SPI5.spi(
            (
                Some(lcd_clk),
                None::<stm32f4xx_hal::gpio::alt::spi5::Miso>,
                Some(lcd_mosi),
            ),
            mode,
            2.MHz(),
            &mut rcc,
        );

        // ExclusiveDevice combines the SPI bus with the chip-select pin
        // and provides the embedded-hal SpiDevice implementation required
        // by display-interface-spi.
        let spi_device = ExclusiveDevice::new_no_delay(lcd_spi, lcd_cs).unwrap();

        let spi_interface = SPIInterface::new(spi_device, lcd_dc);

        let dummy_reset = DummyOutputPin::default();
        let mut delay = dp.TIM1.delay_us(&mut rcc);

        let mut lcd = Ili9341::new(
            spi_interface,
            dummy_reset,
            &mut delay,
            Orientation::PortraitFlipped,
            DisplaySize240x320,
        )
        .unwrap();

        let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::RED);

        Text::with_alignment(
            "First line\nSecond line",
            Point::new(20, 30),
            text_style,
            Alignment::Center,
        )
        .draw(&mut lcd)
        .unwrap();

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
}
