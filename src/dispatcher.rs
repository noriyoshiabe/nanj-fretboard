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

    pub fn dispatch_layout(&self) -> Result<(), JsValue> {
        self._dispatch_layout(Rc::clone(&self.root_view))
    }

    fn _dispatch_layout(&self, parent: Rc<RefCell<dyn View>>) -> Result<(), JsValue> {
        parent.borrow_mut().layout()?;

        for child in parent.borrow().children().iter() {
            self._dispatch_layout(Rc::clone(&child))?;
        }

        Ok(())
    }

    pub fn dispatch_render(&self, ctx: &CanvasRenderingContext2d, dpr: f64, next: &mut bool) -> Result<(), JsValue> {
        self._dispatch_render(Rc::clone(&self.root_view), ctx, dpr, next)
    }

    fn _dispatch_render(&self, parent: Rc<RefCell<dyn View>>, ctx: &CanvasRenderingContext2d, dpr: f64, next: &mut bool) -> Result<(), JsValue> {
        parent.borrow_mut().draw(ctx, dpr, next)?;

        for child in parent.borrow().children().iter() {
            let frame = child.borrow().frame();

            ctx.save();
            ctx.translate(frame.x, frame.y)?;

            self._dispatch_render(Rc::clone(&child), ctx, dpr, next)?;

            ctx.restore();
        }

        Ok(())
    }

    pub fn dispatch_pointer_down(&mut self, x: f64, y: f64) -> Result<(), JsValue> {
        let mut layout = false;

        self.pointing_view = self._dispatch_pointer_down(Rc::clone(&self.root_view), Point { x, y }, &mut layout)?;

        if layout {
            self.dispatch_layout()?;
        }

        Ok(())
    }

    fn _dispatch_pointer_down(&mut self, parent: Rc<RefCell<dyn View>>, p: Point, layout: &mut bool) -> Result<Option<Rc<RefCell<dyn View>>>, JsValue> {
        for child in parent.borrow().children().iter().rev() {
            let frame = child.borrow().frame();
            let local_p = p - parent.borrow().frame().origin();

            if let Some(pointing_view) = self._dispatch_pointer_down(Rc::clone(&child), local_p, layout)? {
                return Ok(Some(pointing_view));
            }

            if frame.contains(local_p) {
                if child.borrow_mut().pointer_down(local_p, layout)? {
                    return Ok(Some(Rc::clone(&child)));
                }
            }
        }

        let frame = parent.borrow().frame();
        if frame.contains(p) {
            if parent.borrow_mut().pointer_down(p - frame.origin(), layout)? {
                return Ok(Some(parent));
            }
        }

        Ok(None)
    }

    pub fn dispatch_pointer_up(&mut self, x: f64, y: f64) -> Result<(), JsValue> {
        if let Some(pointing_view) = &self.pointing_view {
            let mut layout = false;

            let p = self._calc_local_point(Rc::clone(&self.root_view), Rc::clone(&pointing_view), Point { x, y });
            pointing_view.borrow_mut().pointer_up(p, &mut layout)?;

            if layout {
                self.dispatch_layout()?;
            }
        }

        Ok(())
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
