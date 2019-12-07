use piet::kurbo::Rect;
use piet::{
    IntoBrush, 
};
use crate::context;

#[derive(Clone)]
pub enum Brush {
    Solid(u32),
    ////Linear(embedded_graphics::LinearGradient),
    ////Radial(embedded_graphics::RadialGradient),
}

impl<'a> IntoBrush<context::EmbedRenderContext> for Brush {
    fn make_brush<'b>(
        &'b self,
        _piet: &mut context::EmbedRenderContext,
        _bbox: impl FnOnce() -> Rect,
    ) -> Brush {
        self.clone()
    }
}

/* ////
// we call this with different types of gradient that have `add_color_stop_rgba` fns,
// and there's no trait for this behaviour so we use a macro. ¯\_(ツ)_/¯
macro_rules! set_gradient_stops {
    ($dst: expr, $stops: expr) => {
        for stop in $stops {
            let rgba = stop.color.as_rgba_u32();
            $dst.add_color_stop_rgba(
                stop.pos as f64,
                byte_to_frac(rgba >> 24),
                byte_to_frac(rgba >> 16),
                byte_to_frac(rgba >> 8),
                byte_to_frac(rgba),
            );
        }
    };
}
*/ ////