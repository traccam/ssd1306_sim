use chrono::NaiveTime;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, Rectangle, PrimitiveStyle},
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    text::Text,
};
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::text::Baseline;
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};

#[derive(Clone, Default)]
pub struct DisplayState {
    time: NaiveTime,
    lat: f64,
    lon: f64,
    sats: u8,
}


fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 32));

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    let state = DisplayState::default();

    display.clear(BinaryColor::Off).unwrap();

    Text::with_baseline(&heapless::format!(15; "{} {}", state.time, state.sats).unwrap(), Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline(&heapless::format!(10; "N{:.5}", state.lat).unwrap(), Point::new(0, 9), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline(&heapless::format!(10; "E{:.5}", state.lon).unwrap(), Point::new(0, 18), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}