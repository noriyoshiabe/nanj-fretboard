use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::view::{Rect, View};

pub struct Fretboard {
    frame: Rect,
}

impl View for Fretboard {
    fn frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame
    }

    fn layout(&mut self) {
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, dpr: f64, _: &mut bool) -> Result<(), JsValue> {
        ctx.set_stroke_style_str("lightgray");
        ctx.set_line_width(1.0 * dpr);

        ctx.begin_path();

        for i in 0..6 {
            let y = i as f64 * self.frame.height / 5.;
            ctx.move_to(0., y);
            ctx.line_to(self.frame.width, y);
        }

        for i in 2..14 {
            let x = i as f64 * self.frame.width / 13.;
            ctx.move_to(x, 0.);
            ctx.line_to(x, self.frame.height);
        }

        ctx.stroke();

        ctx.set_stroke_style_str("gray");
        ctx.set_line_width(2.0 * dpr);

        ctx.begin_path();

        let x = 1.0 * self.frame.width / 13.;
        ctx.move_to(x, 0.);
        ctx.line_to(x, self.frame.height);

        ctx.stroke();

        ctx.set_fill_style_str("lightgray");

        let y = self.frame.height / 2.;
        let r = self.frame.width * 0.01;

        for i in [3, 5, 7, 9, 12] {
            let x = (i as f64 * 2. + 1.) * self.frame.width / 26.;

            ctx.begin_path();

            if i == 12 {
                ctx.arc(x, self.frame.height / 10. * 3., r, 0., std::f64::consts::PI * 2.)?;
                ctx.fill();

                ctx.begin_path();
                ctx.arc(x, self.frame.height / 10. * 7., r, 0., std::f64::consts::PI * 2.)?;
            } else {
                ctx.arc(x, y, r, 0., std::f64::consts::PI * 2.)?;
            }

            ctx.fill();
        }

        Ok(())
    }
}

impl Fretboard {
    pub fn new() -> Self {
        Self {
            frame: Rect::default(),
        }
    }
}
