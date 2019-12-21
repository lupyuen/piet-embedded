use embedded_graphics::{
    prelude::*,
    fonts::Font as EFont,
    primitives::{
        Line,
        Rectangle,
    },
    pixelcolor::Rgb565, 
    Drawing, 
};
use embedded_hal::{
    digital::v2::OutputPin,
    blocking::{ 
        spi,
        delay::DelayMs,
    },
};
use st7735_lcd::ST7735;

/// Max number of pixels per row
type MaxRowSize = heapless::consts::U20; //// 240;
/// Max number of pixels per block
type MaxBlockSize = heapless::consts::U40; //// 480;

/// Consecutive color words for a row
type RowColors = heapless::Vec::<u16, MaxRowSize>;
/// Consecutive color words for a block
type BlockColors = heapless::Vec::<u16, MaxBlockSize>;

/// Iterator for each row in the pixel data
#[derive(Debug, Clone)]
pub struct RowIterator<P: Iterator<Item = Pixel<Rgb565>>> {
    pixels:      P,
    x_left:      u16,
    x_right:     u16,
    y:           u16,
    colors:      RowColors,
    first_pixel: bool,
}

/// Iterator for each block in the pixel data
#[derive(Debug, Clone)]
pub struct BlockIterator<R: Iterator<Item = PixelRow>> {
    rows:        R,
    x_left:      u16,
    x_right:     u16,
    y_top:       u16,
    y_bottom:    u16,
    colors:      BlockColors,
    first_row:   bool,
}

/// A row of contiguous pixels
pub struct PixelRow {
    pub x_left:  u16,
    pub x_right: u16,
    pub y:       u16,
    pub colors:  RowColors,
}

/// A block of contiguous row
pub struct PixelBlock {
    pub x_left:   u16,
    pub x_right:  u16,
    pub y_top:    u16,
    pub y_bottom: u16,
    pub colors:   BlockColors,
}

/// Draw the pixels as blocks of contiguous pixel rows
pub fn draw_blocks<SPI, DC, RST, T>(display: &mut ST7735<SPI, DC, RST>, item_pixels: T) -> Result<(),()>
where
    SPI: spi::Write<u8>,
    DC: OutputPin,
    RST: OutputPin,
    T: IntoIterator<Item = Pixel<Rgb565>> + Dimensions, {
    let pixels = item_pixels.into_iter();
    let rows = to_rows(pixels);
    let blocks = to_blocks(rows);
    let mut i = 0;
    for PixelBlock { x_left, x_right, y_top, y_bottom, colors, .. } in blocks {
        display.set_pixels(
            x_left, 
            y_top,
            x_right,
            y_bottom,
            colors) ? ;
        //  i += 1; if i > 1000 { break; } ////
    }
    Ok(())
}

/// Batch the pixels into rows of contiguous pixels
fn to_rows<P>(pixels: P) -> RowIterator<P>
where
    P: Iterator<Item = Pixel<Rgb565>>, {
    RowIterator::<P> {
        pixels,
        x_left: 0,
        x_right: 0,
        y: 0,
        colors: RowColors::new(),
        first_pixel: true,
    }
}

/// Batch the rows into blocks, which are contiguous rows
fn to_blocks<R>(rows: R) -> BlockIterator<R>
where
    R: Iterator<Item = PixelRow>, {
    BlockIterator::<R> {
        rows,
        x_left: 0,
        x_right: 0,
        y_top: 0,
        y_bottom: 0,
        colors: BlockColors::new(),
        first_row: true,
    }
}    

impl<P: Iterator<Item = Pixel<Rgb565>>> Iterator for RowIterator<P> {
    type Item = PixelRow;

    /// Return the next row of contiguous pixels
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_pixel = self.pixels.next();
            match next_pixel {
                None => {
                    if self.first_pixel {
                        return None;  //  No pixels to group
                    }                    
                    //  Else return previous pixels as row.
                    let row = PixelRow {
                        x_left: self.x_left,
                        x_right: self.x_right,
                        y: self.y,
                        colors: self.colors.clone(),
                    };
                    self.colors.clear();
                    self.first_pixel = true;
                    return Some(row);
                }
                Some(Pixel(coord, color)) => {
                    let x = coord.0 as u16;
                    let y = coord.1 as u16;
                    let color = color.0;
                    //  Save the first pixel as the row start and handle next pixel.
                    if self.first_pixel {
                        self.first_pixel = false;
                        self.x_left = x;
                        self.x_right = x;
                        self.y = y;
                        self.colors.clear();
                        self.colors.push(color)
                            .expect("never");
                        continue;
                    }
                    //  If this pixel is adjacent to the previous pixel, add to the row.
                    if x == self.x_right + 1 && y == self.y {
                        if self.colors.push(color).is_ok() {
                            //  Don't add pixel if too many pixels in the row.
                            self.x_right = x;
                            continue;
                        }
                    }
                    //  Else return previous pixels as row.
                    let row = PixelRow {
                        x_left: self.x_left,
                        x_right: self.x_right,
                        y: self.y,
                        colors: self.colors.clone(),
                    };
                    self.x_left = x;
                    self.x_right = x;
                    self.y = y;
                    self.colors.clear();
                    self.colors.push(color)
                        .expect("never");
                    return Some(row);
                }
            }
        }
    }
}

impl<R: Iterator<Item = PixelRow>> Iterator for BlockIterator<R> {
    type Item = PixelBlock;

    /// Return the next block of contiguous pixel rows
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_row = self.rows.next();
            match next_row {
                None => {
                    if self.first_row {
                        return None;  //  No rows to group
                    }                    
                    //  Else return previous rows as block.
                    let row = PixelBlock {
                        x_left: self.x_left,
                        x_right: self.x_right,
                        y_top: self.y_top,
                        y_bottom: self.y_bottom,
                        colors: self.colors.clone(),
                    };
                    self.colors.clear();
                    self.first_row = true;
                    return Some(row);
                }
                Some(PixelRow { x_left, x_right, y, colors, .. }) => {
                    //  Save the first row as the block start and handle next block.
                    if self.first_row {
                        self.first_row = false;
                        self.x_left = x_left;
                        self.x_right = x_right;
                        self.y_top = y;
                        self.y_bottom = y;
                        self.colors.clear();
                        self.colors.extend_from_slice(&colors)
                            .expect("never");
                        continue;
                    }
                    //  If this row is adjacent to the previous row and same size, add to the block.
                    if y == self.y_bottom + 1 && x_left == self.x_left && x_right == self.x_right {                        
                        //  Don't add row if too many pixels in the block.
                        if self.colors.extend_from_slice(&colors).is_ok() {
                            self.y_bottom = y;
                            continue;    
                        }
                    }
                    //  Else return previous rows as block.
                    let row = PixelBlock {
                        x_left: self.x_left,
                        x_right: self.x_right,
                        y_top: self.y_top,
                        y_bottom: self.y_bottom,
                        colors: self.colors.clone(),
                    };
                    self.x_left = x_left;
                    self.x_right = x_right;
                    self.y_top = y;
                    self.y_bottom = y;
                    self.colors.clear();
                    self.colors.extend_from_slice(&colors)
                        .expect("never");
                    return Some(row);
                }
            }
        }
    }
}
