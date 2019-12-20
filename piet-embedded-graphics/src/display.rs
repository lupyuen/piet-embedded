use core::fmt::Write;
use arrayvec::ArrayString;
use embedded_graphics::{
    prelude::*,
    fonts,
    pixelcolor::Rgb565,
    primitives::{
        Circle,
        Rectangle,
    },
};
use embedded_hal::{
    self,
    digital::v2::OutputPin,
};
use st7735_lcd::{
    self,
    Orientation,
    ST7735,
};
use mynewt::{
    self,
    result::*,
    hw::hal,
    fill_zero,
};

/* From PineTime Smart Watch wiki: https://wiki.pine64.org/index.php/PineTime
Mynewt SPI port 0 connected to ST7789 display:
LCD_RS (P0.18)	Clock/data pin (CD)
LCD_CS (P0.25)	Chip select
LCD_RESET (P0.26)	Display reset
LCD_BACKLIGHT_{LOW,MID,HIGH} (P0.14, 22, 23)	Backlight (active low)

- Chip select must be held low while driving the display. It must be high when using other SPI devices on the same bus (such as external flash storage) so that the display controller won't respond to the wrong commands.
- SPI must be used in mode 3. Mode 0 (the default) won't work.
- LCD_DISPLAY_* is used to enable the backlight. Set at least one to low to see anything on the screen.
- Use SPI at 8MHz (the fastest clock available on the nRF52832) because otherwise refreshing will be super slow. */

const DISPLAY_SPI: i32  =  0;  //  Mynewt SPI port 0
const DISPLAY_CS: i32   = 25;  //  LCD_CS (P0.25): Chip select
const DISPLAY_DC: i32   = 18;  //  LCD_RS (P0.18): Clock/data pin (CD)
const DISPLAY_RST: i32  = 26;  //  LCD_RESET (P0.26): Display reset
const DISPLAY_HIGH: i32 = 23;  //  LCD_BACKLIGHT_{LOW,MID,HIGH} (P0.14, 22, 23): Backlight (active low)

/// SPI settings for ST7789 display controller
static mut SPI_SETTINGS: hal::hal_spi_settings = hal::hal_spi_settings {
    data_order: hal::HAL_SPI_MSB_FIRST as u8,
    data_mode:  hal::HAL_SPI_MODE3 as u8,  //  SPI must be used in mode 3. Mode 0 (the default) won't work.
    baudrate:   8000,  //  In kHZ. Use SPI at 8MHz (the fastest clock available on the nRF52832) because otherwise refreshing will be super slow.
    word_size:  hal::HAL_SPI_WORD_SIZE_8BIT as u8,
};

/// Initialise the display controller
pub fn start_display() -> MynewtResult<()> {
    //  Create SPI port and GPIO pins
    let mut spi_port = mynewt::SPI::new();
    let mut dc_gpio =  mynewt::GPIO::new();
    let mut rst_gpio = mynewt::GPIO::new();

    //  Init SPI port and GPIO pins
    spi_port.init(
        DISPLAY_SPI, //  Mynewt SPI port 0
        DISPLAY_CS,  //  LCD_CS (P0.25): Chip select
        unsafe { &mut SPI_SETTINGS }
    ) ? ;
    dc_gpio.init(DISPLAY_DC) ? ;    //  LCD_RS (P0.18): Clock/data pin (CD)
    rst_gpio.init(DISPLAY_RST) ? ;  //  LCD_RESET (P0.26): Display reset

    //  Switch on the backlight
    unsafe {
        BACKLIGHT_HIGH = mynewt::GPIO::new();
        BACKLIGHT_HIGH.init(DISPLAY_HIGH) ? ;  //  LCD_BACKLIGHT_{LOW,MID,HIGH} (P0.14, 22, 23): Backlight (active low)
        BACKLIGHT_HIGH.set_low() ? ;    
    }
    
    //  Create display driver
    unsafe { DISPLAY = st7735_lcd::ST7735::new(
        spi_port,    //  SPI Port
        dc_gpio,     //  GPIO Pin for DC
        rst_gpio,    //  GPIO Pin for RST
        true,        //  Whether the display is RGB (true) or BGR (false)
        true         //  Whether the colours are inverted (true) or not (false)
    ) };

    //  Init display driver
    let mut delay = mynewt::Delay::new();
    unsafe {
        DISPLAY.init(&mut delay) ? ;
        DISPLAY.set_orientation(&Orientation::Landscape) ? ;
    }
    Ok(())
}

/// Display the touched (X, Y) coordinates
pub fn show_touch(x: u16, y: u16) -> MynewtResult<()> {
    //  Format coordinates as text into a fixed-size buffer
    let mut buf_x = ArrayString::<[u8; 20]>::new();
    let mut buf_y = ArrayString::<[u8; 20]>::new();
    write!(&mut buf_x, "  X = {}  ", x)
        .expect("show touch fail");
    write!(&mut buf_y, "  Y = {}  ", y)
        .expect("show touch fail");

    //  Prepare the text for rendering
    let text_x = fonts::Font12x16::<Rgb565>
        ::render_str(&buf_x)
        .stroke(Some(Rgb565::from(( 0xff, 0xff, 0xff ))))  //  White
        .fill(Some(Rgb565::from((   0x00, 0x00, 0x00 ))))  //  Black
        .translate(Coord::new(40, 100));
    let text_y = fonts::Font12x16::<Rgb565>
        ::render_str(&buf_y)
        .stroke(Some(Rgb565::from(( 0xff, 0xff, 0xff ))))  //  White
        .fill(Some(Rgb565::from((   0x00, 0x00, 0x00 ))))  //  Black
        .translate(Coord::new(40, 130));
        
    //  Render text to display
    unsafe {
        DISPLAY.draw(text_x);    
        DISPLAY.draw(text_y);    
    }
    Ok(())
}

/// Render the ST7789 display connected to SPI port 0. `start_display()` must have been called earlier.
pub fn test_display() -> MynewtResult<()> {
    //  Create black background
    let background = Rectangle::<Rgb565>
        ::new(Coord::new(0, 0), Coord::new(239, 239))
        .fill(Some(Rgb565::from(( 0x00, 0x00, 0x00 ))));  //  Black

    //  Create circle
    let circle = Circle::<Rgb565>
        ::new(Coord::new(40, 40), 40)
        .fill(Some(Rgb565::from(( 0xff, 0x00, 0xff ))));  //  Magenta

    //  Create text
    let text = fonts::Font12x16::<Rgb565>
        ::render_str("I AM RUSTY BEACON")
        .stroke(Some(Rgb565::from(( 0x00, 0x00, 0x00 ))))  //  Black
        .fill(Some(Rgb565::from((   0xff, 0xff, 0x00 ))))  //  Yellow
        .translate(Coord::new(20, 16));

    //  Render background, circle and text to display
    unsafe {
        super::batch::draw_blocks(&mut DISPLAY, background)
            .expect("draw blocks fail");
        ////DISPLAY.draw(background);
        DISPLAY.draw(circle);
        DISPLAY.draw(text);    
    }
    Ok(())
}

/// Display Driver
pub static mut DISPLAY: Display = fill_zero!(Display);               //  Will be created in `start_display()`
type Display = ST7735<mynewt::SPI, mynewt::GPIO, mynewt::GPIO>;

/// GPIO Pin for Display Backlight
static mut BACKLIGHT_HIGH: mynewt::GPIO = fill_zero!(MynewtGPIO);  //  Will be created in `start_display()`
type MynewtGPIO = mynewt::GPIO;
