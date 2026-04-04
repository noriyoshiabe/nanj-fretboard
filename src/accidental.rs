use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::view::{Rect, View};

pub struct Accidental {
    frame: Rect,
}

impl View for Accidental {
    fn frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame
    }

    fn layout(&mut self) {
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, _: &mut bool) -> Result<(), JsValue> {
        ctx.fill_rect(0.0, 0.0, self.frame.width, self.frame.height);
        Ok(())
    }
}

impl Accidental {
    pub fn new() -> Self {
        Self {
            frame: Rect::default(),
        }
    }
}
