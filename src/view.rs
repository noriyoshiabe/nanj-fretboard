use std::any::{Any};
use std::cell::RefCell;
use std::ops::Sub;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[derive(Debug, Copy, Clone, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x && p.x <= self.x + self.width &&
        p.y >= self.y && p.y <= self.y + self.height
    }

    pub fn origin(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn left(&self) -> f64 {
        self.x
    }

    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    pub fn top(&self) -> f64 {
        self.y
    }

    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }
}

pub trait View: Any {
    fn frame(&self) -> Rect;
    fn set_frame(&mut self, frame: Rect);

    fn bounds(&self) -> Rect {
        Rect { x: 0., y: 0., ..self.frame() }
    }

    fn layout(&mut self) -> Result<(), JsValue> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &CanvasRenderingContext2d, _dpr: f64, _next: &mut bool) -> Result<(), JsValue> {
        Ok(())
    }

    fn pointer_down(&mut self, _p: Point) -> Result<bool, JsValue> {
        Ok(false)
    }

    fn pointer_up(&mut self, _p: Point) -> Result<(), JsValue> {
        Ok(())
    }

    fn children(&self) -> &[Rc<RefCell<dyn View>>] {
        &[]
    }
}
