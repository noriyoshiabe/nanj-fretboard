use std::any::{TypeId};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::view::{Rect, View};
use crate::asset::Asset;
use crate::accidental::Accidental;
use crate::fretboard::Fretboard;
use crate::keyboard::Keyboard;
use crate::question::Question;

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

    fn layout(&mut self) -> Result<(), JsValue> {
        let width = (self.frame.width * 0.25).min(self.frame.height * 0.2);
        let height = width / (10. / 3.);
        let x = (self.frame.width - width) / 2.;

        let mut f_acccidental = Rect { x, y: 0., width, height };

        let h_margin = self.frame.width * 0.025;
        let width = self.frame.width - h_margin * 2.;
        let height = width / (24. / 4.);
        let x = h_margin;

        let mut f_fretboard = Rect { x, y: 0., width, height };

        let h_margin = if self.frame.width < self.frame.height {
            self.frame.width * 0.05
        } else {
            self.frame.width * 0.15
        };
        let width = self.frame.width - h_margin * 2.;
        let height = width / (14. / (4. * 0.95));
        let x = h_margin;

        let mut f_keyboard = Rect { x, y: 0., width, height };

        let empty_height = self.frame.height - f_acccidental.height - f_fretboard.height - f_keyboard.height;
        let v_space = empty_height / 4.;

        f_acccidental.y = v_space;
        f_fretboard.y = f_acccidental.bottom() + v_space;
        f_keyboard.y = f_fretboard.bottom() + v_space;

        for child in self.children.iter() {
            if child.borrow().type_id() == TypeId::of::<Accidental>() {
                child.borrow_mut().set_frame(f_acccidental);
            } else if child.borrow().type_id() == TypeId::of::<Fretboard>() {
                child.borrow_mut().set_frame(f_fretboard);
            } else if child.borrow().type_id() == TypeId::of::<Keyboard>() {
                child.borrow_mut().set_frame(f_keyboard);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, _: f64, _: &mut bool) -> Result<(), JsValue> {
        ctx.clear_rect(0.0, 0.0, self.frame.width, self.frame.height);
        Ok(())
    }

    fn children(&self) -> &[Rc<RefCell<dyn View>>] {
        self.children.as_slice()
    }
}

impl RootView {
    pub fn new(asset: Rc<Asset>, question: Rc<RefCell<Question>>) -> Rc<RefCell<Self>> {
        let mut children: Vec<Rc<RefCell<dyn View>>> = Vec::new();

        let accidental = Accidental::new(Rc::clone(&question));
        let accidental_rc = Rc::clone(&accidental);
        children.push(accidental_rc);

        let keyboard = Keyboard::new(Rc::clone(&question));
        let keyboard_rc = Rc::clone(&keyboard);
        children.push(keyboard_rc);

        let fretboard = Fretboard::new(asset, Rc::clone(&question));
        let fretboard_rc = Rc::clone(&fretboard);
        children.push(fretboard_rc);

        question.borrow_mut().add_observer(accidental);
        question.borrow_mut().add_observer(fretboard);

        Rc::new(RefCell::new(Self {
            frame: Rect::default(),
            children,
        }))
    }
}
