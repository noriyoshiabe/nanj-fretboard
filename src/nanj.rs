use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Performance};

use crate::asset::Asset;
use crate::view::{Rect, View};
use crate::question::{QuestionItem, QuestionEvent, QuestionObserver};

#[derive(Clone, Copy, PartialEq)]
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
    pub question_item: QuestionItem,
}

impl View for NanJ {
    fn frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame;
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, dpr: f64, next: &mut bool) -> Result<(), JsValue> {
        let now = self.performance.now();

        let (source_canvas, translate_y, scale, alpha, next_state) = match self.state {
            NanJState::Normal => {
                (&self.asset.nanj_normal, 0., 1., 1., self.state)
            },
            NanJState::Happy { started_at } => {
                let elapsed = now - started_at;
                let translate_y = elapsed.min(1000.) * 0.2 * -1.;
                let alpha = 1.- elapsed.min(1000.) / 1000.;
                let next_state = if 1000. < elapsed { NanJState::Gone } else { self.state };
                *next = true;

                (&self.asset.nanj_happy, translate_y, 1., alpha, next_state)
            }
            NanJState::Angry { started_at } => {
                let elapsed = now - started_at;
                let scale = 1. + (std::f64::consts::PI * 2. * (elapsed.min(400.) / 400.)).sin().abs() * 0.5;
                
                if elapsed < 400. {
                    *next = true;
                }

                (&self.asset.nanj_angry, 0., scale, 1., self.state)
            }
            NanJState::Gone => return Ok(()),
        };

        let cx = self.frame.width / 2.;
        let cy = self.frame.height / 2.;
        let r = cx * scale;

        ctx.set_global_alpha(alpha);

        ctx.set_stroke_style_str("lightgray");
        ctx.set_fill_style_str("#ddddf0");

        ctx.set_line_width(2.0 * dpr);

        ctx.begin_path();

        ctx.arc(cx, cy + translate_y, r, 0., std::f64::consts::PI * 2.)?;
        ctx.fill();

        ctx.clip();

        let width = self.frame.width * scale;
        let height = self.frame.height * scale;
        let offset_x = (self.frame.width - width) / 2.;
        let offset_y = (self.frame.height - height) / 2.;

        ctx.draw_image_with_html_canvas_element_and_dw_and_dh(&source_canvas, offset_x, offset_y + translate_y, width, height)?;

        ctx.arc(cx, cy + translate_y, r - 1. * dpr, 0., std::f64::consts::PI * 2.)?;
        ctx.stroke();

        if self.state != next_state {
            self.state = next_state;
        }

        Ok(())
    }
}

impl QuestionObserver for NanJ {
    fn on_notify_event(&mut self, event: QuestionEvent) -> Result<(), JsValue> {
        if let QuestionEvent::Answer { correct } = event {
            match self.state {
                NanJState::Normal | NanJState::Angry { started_at: _ } => {
                    if correct {
                        self.state = NanJState::Happy { started_at: self.performance.now() };
                    } else {
                        self.state = NanJState::Angry { started_at: self.performance.now() };
                    }
                },
                _ => {},
            }
        }

        Ok(())
    }
}

impl NanJ {
    pub fn try_new(asset: Rc<Asset>, question_item: QuestionItem) -> Result<Self, JsValue> {
        let performance = web_sys::window().ok_or("window not exists.")?.performance().ok_or("performance not exists.")?;
        let state = NanJState::Normal;

        Ok(Self {
            frame: Rect::default(),
            performance,
            state,
            asset,
            question_item,
        })
    }

    pub fn is_gone(&self) -> bool {
        self.state == NanJState::Gone
    }
}
