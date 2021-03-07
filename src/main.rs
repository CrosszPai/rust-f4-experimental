#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate stm32f4xx_hal as hal;

use arrayvec::ArrayString;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use embedded_graphics::fonts::{Font12x16, Text};
use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Arc,Rectangle};
use embedded_graphics::style::{PrimitiveStyleBuilder, StrokeAlignment, TextStyleBuilder};
use numtoa::NumToA;

use hal::delay::Delay;
use hal::prelude::*;
use hal::spi::{Mode, Phase, Polarity, Spi};
use hal::stm32;
use st7735_lcd::Orientation;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).pclk1(42.mhz()).freeze();
    let gpioa = dp.GPIOA.split();

    // SPI1
    let sck = gpioa.pa5.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();

    let rst = gpioa.pa3.into_push_pull_output();
    let dc = gpioa.pa2.into_push_pull_output();
    let mut cs = gpioa.pa1.into_push_pull_output();
    cs.set_low().unwrap();

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        24.mhz().into(),
        clocks,
    );

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, false, false, 128, 128);
    let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("./ferris.raw"), 86, 64);
    let image: Image<_> = Image::new(&image_raw, Point::new(40, 60));
    let arc_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::WHITE)
        .stroke_width(5)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let text_style = TextStyleBuilder::new(Font12x16)
        .background_color(Rgb565::BLACK)
        .text_color(Rgb565::WHITE)
        .build();
    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();
    disp.set_offset(0, 0);
    image.draw(&mut disp).unwrap();
    let mut progress = 0;
    loop {
        let sweep = progress as f32 * 360.0 / 100.0;
        Rectangle::new(Point::new(0,0), Size::new(64,64))
        .into_styled(PrimitiveStyleBuilder::new().fill_color(Rgb565::BLACK).build())
        .draw(&mut disp).unwrap();
        Arc::new(Point::new(2, 2), 64 - 4, 90.0.deg(), sweep.deg())
            .into_styled(arc_stroke)
            .draw(&mut disp)
            .unwrap();

        // Draw centered text.
        let temp = progress as u32;
        let mut buf = [0u8; 20];
        let mut text = ArrayString::<[_; 10]>::new();
        text.push_str(temp.numtoa_str(10, &mut buf));
        let width = text.as_str().len() as i32 * 12;
        Text::new(&text, Point::new(32 - width / 2, 32 - 16 / 2))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();
        delay.delay_ms(10_u32);
        progress = (progress + 1) % 101;
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic!("err:{}", info);
}
