use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Performance};

use crate::asset::Asset;
use crate::view::{Rect, View};
use crate::question::{QuestionEvent, QuestionObserver};

#[derive(Clone, Copy)]
pub enum NanJState {
    Normal,
    Happy{ started_at: f64 },
    Angry{ started_at: f64 },
    Gone,
}

pub struct NanJ {
    frame: Rect,
    performance: Performance,
    state: NanJState,
    asset: Rc<Asset>,
}

impl View for NanJ {
    fn frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, dpr: f64, next: &mut bool) -> Result<(), JsValue> {
        let source_canvas = match self.state {
            NanJState::Normal => &self.asset.nanj_normal,
            NanJState::Happy { started_at: _ } => &self.asset.nanj_happy,
            NanJState::Angry { started_at: _ } => &self.asset.nanj_angry,
            NanJState::Gone => return Ok(()),
        };

        let y = 0.;

        ctx.set_stroke_style_str("lightgray");
        ctx.set_fill_style_str("#ddddf0");

        ctx.set_line_width(2.0 * dpr);

        ctx.begin_path();

        ctx.arc(self.frame.width / 2., self.frame.height / 2. + y, self.frame.width / 2., 0., std::f64::consts::PI * 2.)?;
        ctx.fill();

        ctx.clip();

        ctx.draw_image_with_html_canvas_element_and_dw_and_dh(&source_canvas, 0.0, y, self.frame.width, self.frame.height)?;

        ctx.arc(self.frame.width / 2., self.frame.height / 2. + y, self.frame.width / 2. - 1. * dpr, 0., std::f64::consts::PI * 2.)?;
        ctx.stroke();

        Ok(())
    }
}

impl QuestionObserver for NanJ {
    fn on_notify_event(&mut self, event: QuestionEvent) -> Result<(), JsValue> {
        if let QuestionEvent::Answer { correct } = event {
            if correct {
                self.state = NanJState::Happy { started_at: self.performance.now() };
            } else {
                self.state = NanJState::Angry { started_at: self.performance.now() };
            }
        }

        Ok(())
    }
}

impl NanJ {
    pub fn try_new(asset: Rc<Asset>) -> Result<Self, JsValue> {
        let performance = web_sys::window().ok_or("window not exists.")?.performance().ok_or("performance not exists.")?;
        let state = NanJState::Normal;

        Ok(Self {
            frame: Rect::default(),
            performance,
            state,
            asset,
        })
    }
}
