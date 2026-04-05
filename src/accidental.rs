use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::view::{Rect, View, Point};

pub struct Accidental {
    frame: Rect,
    active: bool, // temporary
}

impl View for Accidental {
    fn frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, dpr: f64, _: &mut bool) -> Result<(), JsValue> {
        ctx.set_stroke_style_str("gray");
        ctx.set_line_width(1.0 * dpr);

        if !self.active {
           ctx.set_global_alpha(0.5);
        }

        ctx.begin_path();

        let bounds = self.bounds();

        ctx.move_to(bounds.left(), bounds.top());

        ctx.line_to(bounds.right(), bounds.top());
        ctx.line_to(bounds.right(), bounds.bottom());
        ctx.line_to(bounds.left(), bounds.bottom());
        ctx.line_to(bounds.left(), bounds.top());

        ctx.stroke();

        ctx.set_font(&format!("{}px system-ui", dpr * 16.));

        let metrics = ctx.measure_text("#/♭")?;
        let width = metrics.width();
        let height = metrics.actual_bounding_box_ascent();
        let x = (self.frame.width - width) / 2.;
        let y = (self.frame.height - height) / 2. + height;

        ctx.fill_text("#/♭", x, y)?;

        Ok(())
    }

    fn pointer_down(&mut self, _: Point, _: &mut bool) -> Result<bool, JsValue> {
        self.active = true;
        Ok(true)
    }

    fn pointer_up(&mut self, _: Point, _: &mut bool) -> Result<(), JsValue> {
        self.active = false;
        Ok(())
    }

}

impl Accidental {
    pub fn new() -> Self {
        Self {
            frame: Rect::default(),
            active: false,
        }
    }
}
