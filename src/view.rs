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

pub trait View {
    fn frame(&self) -> Rect;
    fn set_frame(&mut self, frame: Rect);

    fn bounds(&self) -> Rect {
        Rect { x: 0., y: 0., ..self.frame() }
    }

    fn layout(&mut self);
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, dpr: f64, next: &mut bool) -> Result<(), JsValue>;

    fn pointer_down(&mut self, _p: Point) -> bool {
        false
    }

    fn pointer_up(&mut self, _p: Point) {}

    fn children(&self) -> &[Rc<RefCell<dyn View>>] {
        &[]
    }

    #[allow(dead_code)]
    fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<dyn View>>> {
        unimplemented!();
    }

    #[allow(dead_code)]
    fn append_child(&mut self, child: Rc<RefCell<dyn View>>) {
        self.children_mut().push(child);
    }

    #[allow(dead_code)]
    fn remove_child(&mut self, child: &Rc<RefCell<dyn View>>) {
        if let Some(index) = self.children().iter().position(|v| Rc::ptr_eq(v, child)) {
            self.children_mut().remove(index);
        }
    }
}
