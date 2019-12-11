use piet::kurbo::{Affine, PathEl, Point, Rect, Shape};
use piet::{
    ////new_error, 
    Color, Error, 
    ////ErrorKind, 
    IntoBrush, 
    RenderContext, StrokeStyle,
};
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
use mynewt::sys::console;
use crate::{ brush, text };
use super::display;

const DISPLAY_WIDTH:  u16 = 240;  //  For PineTime Display
const DISPLAY_HEIGHT: u16 = 240;  //  For PineTime Display

static mut EMBED_TEXT: text::EmbedText = text::EmbedText;

/// Maximum number of transforms supported
type MaxTransforms = heapless::consts::U10;
/// Stack of transforms to be applied to the current render
static mut TRANSFORM_STACK: heapless::Vec<Point, MaxTransforms> = 
    heapless::Vec(heapless::i::Vec::new());

fn get_transform_stack() -> Coord {
    let mut result = Point::ZERO;
    unsafe {
        for p in &mut TRANSFORM_STACK {
            result.x += p.x;
            result.y += p.y;
        }    
    }
    Coord::new(result.x as i32, result.y as i32)
}

pub struct EmbedRenderContext {
    // display: &'a mut Display,
    // text: &'a mut text::EmbedText,
    // phantom: PhantomData<u32 + 'a>,
}

/*
impl<'a> EmbedRenderContext<'a> {
    /// Create a new embedded-graphics back-end.
    ///
    /// At the moment, it uses the "toy text API" for text layout, but when
    /// we change to a more sophisticated text layout approach, we'll probably
    /// need a factory for that as an additional argument.
    pub fn new() -> EmbedRenderContext {
        EmbedRenderContext {
            // display: unsafe { &mut DISPLAY },
            // text: unsafe { &mut EMBED_TEXT },
        }
    }
}
*/

impl RenderContext for EmbedRenderContext {
    type Brush = brush::Brush;
    type Text = text::EmbedText;
    type TextLayout = text::EmbedTextLayout;

    ////type Image = ImageSurface;

    fn status(&mut self) -> Result<(), Error> {
        /* ////  TODO
        let status = self.ctx.status();
        if status == Status::Success {
            Ok(())
        } else {
            let e: Box<dyn std::error::Error> = Box::new(WrappedStatus(status));
            Err(e.into())
        }
        */ ////
        Ok(())
    }

    fn clear(&mut self, color: Color) {
        //  Create brush
        let brush = self.solid_brush(color);
        //  Create rectangle to fill the screen
        let shape = Rect::new(0., 0., 
            DISPLAY_WIDTH  as f64 - 1., 
            DISPLAY_HEIGHT as f64 - 1.);
        //  Fill the screen
        ////self.fill(shape, &brush);
    }

    fn solid_brush(&mut self, color: Color) -> brush::Brush {
        brush::Brush::Solid(color.as_rgba_u32())
    }

    /* ////  TODO
        fn gradient(&mut self, gradient: impl Into<FixedGradient>) -> Result<Brush, Error> {
            match gradient.into() {
                FixedGradient::Linear(linear) => {
                    let (x0, y0) = (linear.start.x, linear.start.y);
                    let (x1, y1) = (linear.end.x, linear.end.y);
                    let lg = embedded_graphics::LinearGradient::new(x0, y0, x1, y1);
                    set_gradient_stops!(&lg, &linear.stops);
                    Ok(Brush::Linear(lg))
                }
                FixedGradient::Radial(radial) => {
                    let (xc, yc) = (radial.center.x, radial.center.y);
                    let (xo, yo) = (radial.origin_offset.x, radial.origin_offset.y);
                    let r = radial.radius;
                    let rg = embedded_graphics::RadialGradient::new(xc + xo, yc + yo, 0.0, xc, yc, r);
                    set_gradient_stops!(&rg, &radial.stops);
                    Ok(Brush::Radial(rg))
                }
            }
        }
    */ ////

    fn fill(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>) {
        let brush = brush.make_brush(self, || shape.bounding_box());
        //  TODO: Handle Bezier path
        //  self.set_path(shape);

        //  TODO: For now we fill the bounding box
        let bounding_box = shape.bounding_box();
        let left_top = Coord::new(bounding_box.x0 as i32, bounding_box.y0 as i32);
        let right_btm = Coord::new(bounding_box.x1 as i32, bounding_box.y1 as i32);

        //  Get fill color
        let fill = self.convert_brush(&brush);

        //  Create rectangle with fill
        let rect = Rectangle::<Rgb565>
            ::new(left_top, right_btm)
            .fill(Some(fill))
            .translate(get_transform_stack())
            ;
        unsafe { display::DISPLAY.draw(rect); }

        ////self.ctx.set_fill_rule(embedded_graphics::FillRule::Winding);
        ////self.ctx.fill();
    }

    fn fill_even_odd(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>) {
        self.fill(shape, brush);
        /* TODO
        let brush = brush.make_brush(self, || shape.bounding_box());
        self.set_path(shape);
        self.set_brush(&brush);
        self.ctx.set_fill_rule(embedded_graphics::FillRule::EvenOdd);
        self.ctx.fill();
        */
    }

    fn clip(&mut self, _shape: impl Shape) {
        console::print("no clip\n");  ////  TODO
        /*
        self.set_path(shape);
        self.ctx.set_fill_rule(embedded_graphics::FillRule::Winding);
        self.ctx.clip();
        */
    }

    fn stroke(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>, width: f64) {
        let brush = brush.make_brush(self, || shape.bounding_box());

        //  Get stroke color
        let stroke = self.convert_brush(&brush);

        //  Draw a line for each segment of the Bezier path
        let mut first: Option<Point> = None;
        let mut last = Point::ZERO;
        for el in shape.to_bez_path(0.1) {  //  Previously 1e-3
            match el {
                PathEl::MoveTo(p) => {
                    ////self.ctx.move_to(p.x, p.y);
                    if (first.is_none()) { first = Some(p); }
                    last = p;
                }
                PathEl::LineTo(p) => {
                    //  Draw line from last to p with styled stroke
                    let last_coord = Coord::new(last.x as i32, last.y as i32);
                    let p_coord = Coord::new(p.x as i32, p.y as i32);
                    let line = Line::<Rgb565>
                        ::new(last_coord, p_coord)
                        .stroke(Some(stroke))
                        .stroke_width(width as u8)
                        .translate(get_transform_stack())
                        ;
                    unsafe { display::DISPLAY.draw(line); }
                    ////self.ctx.line_to(p.x, p.y);
                    if (first.is_none()) { first = Some(p); }
                    last = p;
                }
                PathEl::QuadTo(p1, p2) => {
                    //  TODO: Handle quad
                    //  Draw line from last to p2 with styled stroke
                    let last_coord = Coord::new(last.x as i32, last.y as i32);
                    let p2_coord = Coord::new(p2.x as i32, p2.y as i32);
                    let line = Line::<Rgb565>
                        ::new(last_coord, p2_coord)
                        .stroke(Some(stroke))
                        .stroke_width(width as u8)
                        .translate(get_transform_stack())
                        ;
                    unsafe { display::DISPLAY.draw(line); }
                    ////let q = QuadBez::new(last, p1, p2);
                    ////let c = q.raise();
                    ////self.ctx
                        ////.curve_to(c.p1.x, c.p1.y, c.p2.x, c.p2.y, p2.x, p2.y);
                    if (first.is_none()) { first = Some(p1); }
                    last = p2;
                }
                PathEl::CurveTo(p1, _p2, p3) => {
                    //  TODO: Handle curve
                    //  Draw line from last to p3 with styled stroke
                    let last_coord = Coord::new(last.x as i32, last.y as i32);
                    let p3_coord = Coord::new(p3.x as i32, p3.y as i32);
                    let line = Line::<Rgb565>
                        ::new(last_coord, p3_coord)
                        .stroke(Some(stroke))
                        .stroke_width(width as u8)
                        .translate(get_transform_stack())
                        ;
                    unsafe { display::DISPLAY.draw(line); }
                    ////self.ctx.curve_to(p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
                    if (first.is_none()) { first = Some(p1); }
                    last = p3;
                }
                PathEl::ClosePath => {
                    let first = first.expect("close path fail");
                    let last_coord = Coord::new(last.x as i32, last.y as i32);
                    let first_coord = Coord::new(first.x as i32, first.y as i32);
                    let line = Line::<Rgb565>
                        ::new(last_coord, first_coord)
                        .stroke(Some(stroke))
                        .stroke_width(width as u8)
                        .translate(get_transform_stack())
                        ;
                    unsafe { display::DISPLAY.draw(line); }
                    ////self.ctx.close_path()
                }
            }
        }

        ////self.ctx.stroke();
    }

    fn stroke_styled(
        &mut self,
        shape: impl Shape,
        brush: &impl IntoBrush<Self>,
        width: f64,
        _style: &StrokeStyle,
    ) {
        self.stroke(shape, brush, width);
        /* TODO
            let brush = brush.make_brush(self, || shape.bounding_box());
            self.set_path(shape);
            self.set_stroke(width, Some(style));
            self.set_brush(&brush);
            self.ctx.stroke();
        */
    }

    fn text(&mut self) -> &mut Self::Text {
        unsafe { &mut EMBED_TEXT }
    }

    fn draw_text(
        &mut self,
        layout: &Self::TextLayout,
        pos: impl Into<Point>,
        brush: &impl IntoBrush<Self>,
    ) {
        let brush = brush.make_brush(self, || Rect::ZERO);
        let pos = pos.into();

        //  Get stroke color
        let stroke = self.convert_brush(&brush);

        //  Create text
        let text = embedded_graphics::fonts::Font12x16::<Rgb565>
            ::render_str(&layout.text)
            .stroke(Some(stroke))
            .fill(Some(Rgb565::from((   0x00, 0x00, 0x00 ))))  //  TODO: Remove black background fill
            .translate(Coord::new(pos.x as i32, pos.y as i32))
            .translate(get_transform_stack())
            ;
        
        //  Render text to display
        unsafe { display::DISPLAY.draw(text); }

        // TODO: bounding box for text
        /*
        self.ctx.set_scaled_font(&layout.font);
        self.set_brush(&brush);
        self.ctx.move_to(pos.x, pos.y);
        self.ctx.show_text(&layout.text);
        */
    }

    fn save(&mut self) -> Result<(), Error> {
        unsafe {
            TRANSFORM_STACK.push(Point::ZERO)
                .expect("transform stack overflow");
        }
        Ok(())
    }

    fn restore(&mut self) -> Result<(), Error> {
        unsafe { 
            TRANSFORM_STACK.pop()
                .expect("transform stack empty"); 
        }
        Ok(())
    }

    fn finish(&mut self) -> Result<(), Error> {
        self.status()
    }

    fn transform(&mut self, _transform: Affine) {
        console::print("transform ");  ////  TODO
        for f in &_transform.0 {
            console::printint(*f as i32);            
            console::print(", ");            
        }
        console::print("\n"); // console::flush();            

        unsafe {
            let mut point = TRANSFORM_STACK.pop()
                .expect("transform stack empty");
            point.x += &_transform.0[4];
            point.y += &_transform.0[5];
            TRANSFORM_STACK.push(point)
                .expect("never");
        }

        //cortex_m::asm::bkpt(); ////
        ////self.ctx.transform(affine_to_matrix(transform));        
    }
}

/*
    fn convert_line_cap(line_cap: LineCap) -> embedded_graphics::LineCap {
        match line_cap {
            LineCap::Butt => embedded_graphics::LineCap::Butt,
            LineCap::Round => embedded_graphics::LineCap::Round,
            LineCap::Square => embedded_graphics::LineCap::Square,
        }
    }

    fn convert_line_join(line_join: LineJoin) -> embedded_graphics::LineJoin {
        match line_join {
            LineJoin::Miter => embedded_graphics::LineJoin::Miter,
            LineJoin::Round => embedded_graphics::LineJoin::Round,
            LineJoin::Bevel => embedded_graphics::LineJoin::Bevel,
        }
    }
*/

impl EmbedRenderContext {
    /// Get the source pattern for the brush
    fn convert_brush(&mut self, brush: &brush::Brush) -> Rgb565 {
        match *brush {
            brush::Brush::Solid(rgba) => {
                Rgb565::from((
                    (rgba >> 24) as u8,  //  Red
                    (rgba >> 16) as u8,  //  Green
                    (rgba >>  8) as u8   //  Blue
                ))  //  Alpha transparency not used: rgba as u8
            }
            ////Brush::Linear(ref linear) => self.ctx.set_source(linear),
            ////Brush::Radial(ref radial) => self.ctx.set_source(radial),
        }
    }

    /* TODO
        /// Set the stroke parameters.
        fn set_stroke(&mut self, width: f64, style: Option<&StrokeStyle>) {
            self.ctx.set_line_width(width);

            let line_join = style
                .and_then(|style| style.line_join)
                .unwrap_or(LineJoin::Miter);
            self.ctx.set_line_join(convert_line_join(line_join));

            let line_cap = style
                .and_then(|style| style.line_cap)
                .unwrap_or(LineCap::Butt);
            self.ctx.set_line_cap(convert_line_cap(line_cap));

            let miter_limit = style.and_then(|style| style.miter_limit).unwrap_or(10.0);
            self.ctx.set_miter_limit(miter_limit);

            match style.and_then(|style| style.dash.as_ref()) {
                None => self.ctx.set_dash(&[], 0.0),
                Some((dashes, offset)) => self.ctx.set_dash(dashes, *offset),
            }
        }

        fn set_path(&mut self, shape: impl Shape) {
            // This shouldn't be necessary, we always leave the context in no-path
            // state. But just in case, and it should be harmless.
            ////self.ctx.new_path();
            let mut last = Point::ZERO;
            for el in shape.to_bez_path(0.1) {  //  Previously 1e-3
                match el {
                    PathEl::MoveTo(p) => {
                        ////self.ctx.move_to(p.x, p.y);
                        last = p;
                    }
                    PathEl::LineTo(p) => {
                        ////self.ctx.line_to(p.x, p.y);
                        last = p;
                    }
                    PathEl::QuadTo(p1, p2) => {
                        let q = QuadBez::new(last, p1, p2);
                        let c = q.raise();
                        ////self.ctx
                            ////.curve_to(c.p1.x, c.p1.y, c.p2.x, c.p2.y, p2.x, p2.y);
                        last = p2;
                    }
                    PathEl::CurveTo(p1, p2, p3) => {
                        ////self.ctx.curve_to(p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
                        last = p3;
                    }
                    PathEl::ClosePath => {
                        ////self.ctx.close_path()
                    }
                }
            }
        }
    */
}

/* ////
    fn byte_to_frac(byte: u32) -> f64 {
        ((byte & 255) as f64) * (1.0 / 255.0)
    }

    /// Can't implement RoundFrom here because both types belong to other crates.
    fn affine_to_matrix(affine: Affine) -> Matrix {
        let a = affine.as_coeffs();
        Matrix {
            xx: a[0],
            yx: a[1],
            xy: a[2],
            yy: a[3],
            x0: a[4],
            y0: a[5],
        }
    }

    fn scale_matrix(scale: f64) -> Matrix {
        Matrix {
            xx: scale,
            yx: 0.0,
            xy: 0.0,
            yy: scale,
            x0: 0.0,
            y0: 0.0,
        }
    }
*/ ////