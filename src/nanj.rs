use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Performance};

use crate::asset::Asset;
use crate::view::{Point, Rect, View};

pub struct NanJ {
    frame: Rect,
    source_canvas: HtmlCanvasElement,
    performance: Performance,
    started_at: f64,
    asset: Rc<Asset>,
}

impl View for NanJ {
    fn frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame
    }

    fn layout(&mut self) -> Result<(), JsValue> {
        self.source_canvas = self.asset.nanj_normal.source_canvas(self.frame.width, self.frame.height)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, dpr: f64, next: &mut bool) -> Result<(), JsValue> {
        let elapsed = self.performance.now() - self.started_at;

        let y = elapsed.min(1000.0) * 0.1;

        ctx.set_stroke_style_str("lightgray");
        ctx.set_fill_style_str("#ddddf0");

        ctx.set_line_width(2.0 * dpr);

        ctx.begin_path();

        ctx.arc(self.frame.width / 2., self.frame.height / 2. + y, self.frame.width / 2., 0., std::f64::consts::PI * 2.)?;
        ctx.fill();

        ctx.clip();

        ctx.draw_image_with_html_canvas_element_and_dw_and_dh(&self.source_canvas, 0.0, y, self.frame.width, self.frame.height)?;

        ctx.arc(self.frame.width / 2., self.frame.height / 2. + y, self.frame.width / 2. - 1. * dpr, 0., std::f64::consts::PI * 2.)?;
        ctx.stroke();

        if 1000.0 > elapsed {
            *next = true
        }

        Ok(())
    }

    fn pointer_down(&mut self, _: Point, _: &mut bool) -> Result<bool, JsValue> {
        self.started_at = self.performance.now();
        Ok(true)
    }

    fn pointer_up(&mut self, _: Point, _: &mut bool) -> Result<(), JsValue> {
        self.started_at = self.performance.now();
        Ok(())
    }
}

impl NanJ {
    pub fn try_new(asset: Rc<Asset>) -> Result<Self, JsValue> {
        let source_canvas = asset.nanj_normal.source_canvas(0., 0.)?;
        let performance = web_sys::window().ok_or("window not exists.")?.performance().ok_or("performance not exists.")?;
        let started_at = performance.now();

        Ok(Self {
            frame: Rect::default(),
            source_canvas,
            performance,
            started_at,
            asset,
        })
    }
}
