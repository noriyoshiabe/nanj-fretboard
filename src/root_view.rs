use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::view::{Rect, View};

pub struct RootView {
    frame: Rect,
    children: Vec<Rc<RefCell<dyn View>>>,
}

impl View for RootView {
    fn frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame
    }

    fn layout(&mut self) {
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, _: &mut bool) -> Result<(), JsValue> {
        ctx.clear_rect(0.0, 0.0, self.frame.width, self.frame.height);
        Ok(())
    }

    fn children(&self) -> &[Rc<RefCell<dyn View>>] {
        self.children.as_slice()
    }

    fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<dyn View>>> {
        &mut self.children
    }
}

impl RootView {
    pub fn new() -> Self {
        Self {
            frame: Rect::default(),
            children: Vec::new(),
        }
    }
}
