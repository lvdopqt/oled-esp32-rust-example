use std::thread::sleep;
use std::time::Duration;

use esp_idf_hal::prelude::*;
use esp_idf_hal::i2c::{I2cDriver, I2cConfig};
use esp_idf_sys as _;

use ssd1306::{
    prelude::*,
    I2CDisplayInterface,
    Ssd1306,
    mode::BufferedGraphicsMode,
    size::DisplaySize128x64,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio32, // SDA
        peripherals.pins.gpio33, // SCL
        &I2cConfig::new().baudrate(400.kHz().into()),
    )?;

    let interface = I2CDisplayInterface::new(i2c);

    let mut display: Ssd1306<_, DisplaySize128x64, BufferedGraphicsMode<_>> =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

    display.init().map_err(|e| anyhow::anyhow!("Display init failed: {:?}", e))?;

    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

    let mut x = 0;
    let mut dx = 2;

    loop {
        display.clear(BinaryColor::Off).map_err(|e| anyhow::anyhow!("Clear failed: {:?}", e))?;

        Text::new("HELLO WORLD", Point::new(x, 32), style)
            .draw(&mut display)
            .map_err(|e| anyhow::anyhow!("Draw failed: {:?}", e))?;

        display.flush().map_err(|e| anyhow::anyhow!("Flush failed: {:?}", e))?;

        // Animate: move back and forth
        x += dx;
        if x > 60 || x < -60 {
            dx = -dx;
        }

        sleep(Duration::from_millis(100));
    }
}
