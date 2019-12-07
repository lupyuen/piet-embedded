//! The embedded-graphics backend for the Piet 2D graphics abstraction.

#![no_std] ////
#![feature(const_transmute)] ////

mod brush;
mod context;
mod display;
////mod grapheme;
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