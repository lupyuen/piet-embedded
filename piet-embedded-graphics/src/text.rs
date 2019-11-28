use unicode_segmentation::UnicodeSegmentation;
use arrayvec::ArrayString;
use crate::grapheme::point_x_in_grapheme;
use piet::kurbo::{ Point, };
use piet::{
    Error, 
    Font, FontBuilder, HitTestMetrics,
    HitTestPoint, HitTestTextPosition, 
    Text, TextLayout, TextLayoutBuilder,
};
use embedded_graphics::{
    fonts,
    pixelcolor::Rgb565, 
};

pub type FontType = fonts::Font12x16::<'static, Rgb565>;
pub const FONT_WIDTH:  u16 = 12;
pub const FONT_HEIGHT: u16 = 16;

/// Right now, we don't need any state, as the "toy text API" treats the
/// access to system font information as a global. This will change.
pub struct EmbedText;

pub struct EmbedFont;

pub struct EmbedFontBuilder {
    ////family: String,
    ////weight: FontWeight,
    ////slant: FontSlant,
    ////size: f64,
}

pub struct EmbedTextLayout {
    ////font: ScaledFont,
    pub text: ArrayString::<[u8; 20]>,
}

pub struct EmbedTextLayoutBuilder(EmbedTextLayout);

impl EmbedText {
    /// Create a new factory that satisfies the piet `Text` trait.
    ///
    /// No state is needed for now because the current implementation is just
    /// toy text, but that will change when proper text is implemented.
    pub fn new() -> EmbedText {
        EmbedText
    }
}

impl Text for EmbedText {
    type Font = EmbedFont;
    type FontBuilder = EmbedFontBuilder;
    type TextLayout = EmbedTextLayout;
    type TextLayoutBuilder = EmbedTextLayoutBuilder;

    fn new_font_by_name(&mut self, name: &str, size: f64) -> Self::FontBuilder {
        EmbedFontBuilder {
            ////family: name,
            ////size: size.round_into(),
            ////weight: FontWeight::Normal,
            ////slant: FontSlant::Normal,
        }
    }

    fn new_text_layout(&mut self, font: &Self::Font, text: &str) -> Self::TextLayoutBuilder {
        let text_layout = EmbedTextLayout {
            ////font: font.0.clone(),
            text: text,
        };
        EmbedTextLayoutBuilder(text_layout)
    }
}


impl FontBuilder for EmbedFontBuilder {
    type Out = EmbedFont;

    fn build(self) -> Result<Self::Out, Error> {
        Ok(EmbedFont)
        /*
        let font_face = FontFace::toy_create(&self.family, self.slant, self.weight);
        let font_matrix = scale_matrix(self.size);
        let ctm = scale_matrix(1.0);
        let options = FontOptions::default();
        let scaled_font = ScaledFont::new(&font_face, &font_matrix, &ctm, &options);
        Ok(EmbedFont(scaled_font))
        */
    }
}

impl Font for EmbedFont {}

impl TextLayoutBuilder for EmbedTextLayoutBuilder {
    type Out = EmbedTextLayout;

    fn build(self) -> Result<Self::Out, Error> {
        Ok(self.0)
    }
}

impl TextLayout for EmbedTextLayout {
    fn width(&self) -> f64 {
        self.font.text_extents(&self.text).x_advance
    }

    // first assume one line.
    // TODO do with lines
    fn hit_test_point(&self, point: Point) -> HitTestPoint {
        // internal logic is using grapheme clusters, but return the text position associated
        // with the border of the grapheme cluster.

        // null case
        if self.text.len() == 0 {
            return HitTestPoint::default();
        }

        // get bounds
        // TODO handle if string is not null yet count is 0?
        let end = UnicodeSegmentation::graphemes(self.text.as_str(), true).count() - 1;
        let end_bounds = match self.get_grapheme_boundaries(end) {
            Some(bounds) => bounds,
            None => return HitTestPoint::default(),
        };

        let start = 0;
        let start_bounds = match self.get_grapheme_boundaries(start) {
            Some(bounds) => bounds,
            None => return HitTestPoint::default(),
        };

        // first test beyond ends
        if point.x > end_bounds.trailing {
            let mut res = HitTestPoint::default();
            res.metrics.text_position = self.text.len();
            return res;
        }
        if point.x <= start_bounds.leading {
            return HitTestPoint::default();
        }

        // then test the beginning and end (common cases)
        if let Some(hit) = point_x_in_grapheme(point.x, &start_bounds) {
            return hit;
        }
        if let Some(hit) = point_x_in_grapheme(point.x, &end_bounds) {
            return hit;
        }

        // Now that we know it's not beginning or end, begin binary search.
        // Iterative style
        let mut left = start;
        let mut right = end;
        loop {
            // pick halfway point
            let middle = left + ((right - left) / 2);

            let grapheme_bounds = match self.get_grapheme_boundaries(middle) {
                Some(bounds) => bounds,
                None => return HitTestPoint::default(),
            };

            if let Some(hit) = point_x_in_grapheme(point.x, &grapheme_bounds) {
                return hit;
            }

            // since it's not a hit, check if closer to start or finish
            // and move the appropriate search boundary
            if point.x < grapheme_bounds.leading {
                right = middle;
            } else if point.x > grapheme_bounds.trailing {
                left = middle + 1;
            } else {
                unreachable!("hit_test_point conditional is exhaustive");
            }
        }
    }

    fn hit_test_text_position(&self, text_position: usize) -> Option<HitTestTextPosition> {
        // Using substrings, but now with unicode grapheme awareness

        let text_len = self.text.len();

        if text_position == 0 {
            return Some(HitTestTextPosition::default());
        }

        if text_position as usize >= text_len {
            return Some(HitTestTextPosition {
                point: Point {
                    x: self.font.text_extents(&self.text).x_advance,
                    y: 0.0,
                },
                metrics: HitTestMetrics {
                    text_position: text_len,
                },
            });
        }

        // Already checked that text_position > 0 and text_position < count.
        // If text position is not at a grapheme boundary, use the text position of current
        // grapheme cluster. But return the original text position
        // Use the indices (byte offset, which for our purposes = utf8 code units).
        let grapheme_indices = UnicodeSegmentation::grapheme_indices(self.text.as_str(), true)
            .take_while(|(byte_idx, _s)| text_position >= *byte_idx);

        if let Some((byte_idx, _s)) = grapheme_indices.last() {
            let point_x = self.font.text_extents(&self.text[0..byte_idx]).x_advance;

            Some(HitTestTextPosition {
                point: Point { x: point_x, y: 0.0 },
                metrics: HitTestMetrics {
                    text_position: text_position,
                },
            })
        } else {
            // iterated to end boundary
            Some(HitTestTextPosition {
                point: Point {
                    x: self.font.text_extents(&self.text).x_advance,
                    y: 0.0,
                },
                metrics: HitTestMetrics {
                    text_position: text_len,
                },
            })
        }
    }
}