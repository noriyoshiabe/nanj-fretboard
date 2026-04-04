use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};

use crate::view::{Point, View};

pub struct Dispatcher {
    root_view: Rc<RefCell<dyn View>>,
    pointing_view: Option<Rc<RefCell<dyn View>>>,
}

impl Dispatcher {
    pub fn new(root_view: Rc<RefCell<dyn View>>) -> Self {
        Self {
            root_view,
            pointing_view: None,
        }
    }

    pub fn dispatch_layout(&self) {
        self._dispatch_layout(Rc::clone(&self.root_view));
    }

    fn _dispatch_layout(&self, parent: Rc<RefCell<dyn View>>) {
        parent.borrow_mut().layout();

        for child in parent.borrow().children().iter() {
            self._dispatch_layout(Rc::clone(&child));
        }
    }

    pub fn dispatch_render(&self, ctx: &CanvasRenderingContext2d, next: &mut bool) -> Result<(), JsValue> {
        self._dispatch_render(Rc::clone(&self.root_view), ctx, next)
    }

    fn _dispatch_render(&self, parent: Rc<RefCell<dyn View>>, ctx: &CanvasRenderingContext2d, next: &mut bool) -> Result<(), JsValue> {
        parent.borrow_mut().draw(ctx, next)?;

        for child in parent.borrow().children().iter() {
            let frame = child.borrow().frame();

            ctx.save();
            ctx.translate(frame.x, frame.y)?;

            self._dispatch_render(Rc::clone(&child), ctx, next)?;

            ctx.restore();
        }

        Ok(())
    }

    pub fn dispatch_pointer_down(&mut self, x: f64, y: f64) {
        self.pointing_view = self._dispatch_pointer_down(Rc::clone(&self.root_view), Point { x, y });
    }

    fn _dispatch_pointer_down(&mut self, parent: Rc<RefCell<dyn View>>, p: Point) -> Option<Rc<RefCell<dyn View>>> {
        for child in parent.borrow().children().iter().rev() {
            let frame = child.borrow().frame();
            let local_p = p - parent.borrow().frame().origin();

            if let Some(pointing_view) = self._dispatch_pointer_down(Rc::clone(&child), local_p) {
                return Some(pointing_view);
            }

            if frame.contains(local_p) {
                if child.borrow_mut().pointer_down(local_p) {
                    return Some(Rc::clone(&child));
                }
            }
        }

        let frame = parent.borrow().frame();
        if frame.contains(p) {
            if parent.borrow_mut().pointer_down(p - frame.origin()) {
                return Some(parent);
            }
        }

        None
    }

    pub fn dispatch_pointer_up(&mut self, x: f64, y: f64) {
        if let Some(pointing_view) = &self.pointing_view {
            let p = self._calc_local_point(Rc::clone(&self.root_view), Rc::clone(&pointing_view), Point { x, y });
            pointing_view.borrow_mut().pointer_up(p);
        }
    }

    fn _calc_local_point(&self, parent: Rc<RefCell<dyn View>>, target: Rc<RefCell<dyn View>>, p: Point) -> Point {
        if Rc::ptr_eq(&parent, &target) {
            return p - parent.borrow().frame().origin();
        }

        for child in parent.borrow().children().iter() {
            let frame = child.borrow().frame();
            let local_p = p - parent.borrow().frame().origin();

            if Rc::ptr_eq(child, &target) {
                return local_p - frame.origin();
            } else {
                return self._calc_local_point(Rc::clone(&child), target, local_p);
            }
        }

        p
    }
}
