//! The embedded-graphics backend for the Piet 2D graphics abstraction.

#![no_std]

mod batch;
mod brush;
mod context;
mod display;
mod image;
mod status;
mod text;

#[cfg(test)]
mod test;

pub use context::EmbedRenderContext;
pub use brush::Brush;
pub use text::{
    EmbedFont,
    EmbedFontBuilder,
    EmbedText,
    EmbedTextLayout,
    EmbedTextLayoutBuilder,
};
pub use display::{ start_display, draw_to_display, set_display_pixels, show_touch };